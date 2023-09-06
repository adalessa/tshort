pub mod session {
    use crate::project::Project;
    use tmux_interface::{HasSession, NewSession, SwitchClient, Tmux};

    pub fn exists(session_name: &str) -> bool {
        Tmux::with_command(HasSession::new().target_session(session_name))
            .status()
            .unwrap()
            .success()
    }

    pub fn connect(session_name: &str) -> bool {
        Tmux::with_command(SwitchClient::new().target_session(session_name))
            .status()
            .unwrap()
            .success()
    }

    pub fn create(item: Project) -> bool {
        if !exists(&item.session_name()) {
            Tmux::with_command(
                NewSession::new()
                    .detached()
                    .session_name(item.tmux_display())
                    .start_directory(item.path().to_str().unwrap())
                    .shell_command(item.get_command().cmd)
            )
            .output()
            .unwrap();
        }

        true
    }

    pub fn connect_or_create(item: Project) -> bool {
        create(item.to_owned());
        connect(&item.tmux_display())
    }
}
