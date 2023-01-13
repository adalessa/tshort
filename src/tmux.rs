pub mod session {

    use crate::project::selector::Project;
    use tmux_interface::{NewSession, TmuxCommand};

    pub struct SessionManager {
    }

    impl SessionManager {
        pub fn new() -> Self {
            Self { }
        }

        pub fn session_exists(&self, session_name: &str) -> bool {
            TmuxCommand::new()
                .has_session()
                .target_session(session_name)
                .output()
                .unwrap()
                .success()
        }

        pub fn create_or_connect(&self, item: Project) -> bool {
            self.create(item.to_owned());
            self.connect(&item.session_name())
        }

        pub fn connect(&self, session_name: &str) -> bool {
            TmuxCommand::new()
                .switch_client()
                .target_session(session_name)
                .output()
                .unwrap()
                .success()
        }

        pub fn create(&self, item: Project) -> bool {
            let tmux = TmuxCommand::new();

            let has_session = tmux
                .has_session()
                .target_session(item.session_name())
                .output()
                .unwrap()
                .success();

            if !has_session {
                return NewSession::new()
                    .session_name(item.session_name())
                    .detached()
                    .start_directory(item.path().to_str().unwrap())
                    .shell_command("$EDITOR")
                    .output()
                    .unwrap()
                    .success();
            }

            return has_session;
        }
    }
}
