use std::collections::{HashMap, HashSet};

use gpui_kit::{
    App, Context, FocusHandle, Focusable, IntoElement, ParentElement, Render, Styled, Window, div,
};

use crate::{config, game_shell::GameShell, games};

pub struct MainView {
    focus_handle: FocusHandle,
    shells: HashMap<String, GameShell>, // 惰性加载：首次切到某游戏才创建，切走不销毁
    errors: HashMap<String, String>,    // 加载失败记录（不自动重试）
    loading: HashSet<String>,           // 进行中的加载（防重复发起）
}

impl MainView {
    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe_global::<config::ConfigStore>(|this, cx| {
            // 清理已被删除的游戏的 shell/错误/加载状态（drop GameShell 即卸载其 VM）
            let ids: HashSet<String> = config::current(cx)
                .games
                .iter()
                .map(|g| g.id.clone())
                .collect();
            this.shells.retain(|id, _| ids.contains(id));
            this.errors.retain(|id, _| ids.contains(id));
            this.loading.retain(|id| ids.contains(id));
            this.ensure_current_shell(cx);
            cx.notify();
        })
        .detach();
        Self {
            focus_handle: cx.focus_handle(),
            shells: HashMap::new(),
            errors: HashMap::new(),
            loading: HashSet::new(),
        }
    }

    /// 保证当前游戏已发起加载。
    ///
    /// shell 的创建必须在**浅栈**处进行（异步任务，帧间执行）：
    /// QuickJS VM + cranelift JIT 初始化吃栈，在 render 调用栈里做会栈溢出。
    pub fn ensure_current_shell(&mut self, cx: &mut Context<Self>) {
        let Some(game) = games::resolve(cx) else {
            return;
        };
        let id = game.id.clone();
        if self.shells.contains_key(&id)
            || self.errors.contains_key(&id)
            || !self.loading.insert(id.clone())
        {
            return;
        }

        cx.spawn(async move |this, cx| {
            let result = this
                .update_in(cx, |_, window, cx| GameShell::load(&game, window, cx))
                .and_then(|inner| inner);
            this.update(cx, |this, cx| {
                this.loading.remove(&id);
                match result {
                    Ok(shell) => {
                        this.shells.insert(id, shell);
                    }
                    Err(err) => {
                        this.errors.insert(id, err.to_string());
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}

impl Focusable for MainView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for MainView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(game) = games::resolve(cx) else {
            return div().size_full().p_4().child("尚未导入游戏：点击侧栏右上角 + 导入");
        };

        let body = if let Some(err) = self.errors.get(&game.id) {
            div()
                .child(format!(
                    "插件加载失败：{err}\n请确认目录 shell/{}/main.js 存在",
                    game.id
                ))
                .into_any_element()
        } else if let Some(shell) = self.shells.get(&game.id) {
            div().size_full().child(shell.view()).into_any_element()
        } else {
            div().child("加载中…").into_any_element()
        };

        div().size_full().p_4().child(body)
    }
}
