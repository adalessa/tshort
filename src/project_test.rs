#[cfg(test)]
pub mod project_test {
    use std::borrow::Cow;

    use crate::{project::Project, utils::config::ProjectConfig};

    #[test]
    fn test_can_generate_the_names() {
        let config: ProjectConfig = ProjectConfig {
            name: "PHP".to_string(),
            directory: "/home/user/code/php".to_string(),
            icon: Some("".to_string()),
            color: None,
        };
        let project: Project = Project::new(
            "/home/user/code/php/my-project".to_string(),
            &config,
        );

        assert_eq!(Cow::from("[PHP] my-project"), project.session_name());
        assert_eq!(Cow::from("  my-project"), project.tmux_display());
        assert_eq!(Cow::from("  my-project"), project.skim_display());
        assert_eq!(Cow::from("[PHP] my-project"), project.skim_text());
    }

    #[test]
    fn test_can_generate_the_names_with_color() {
        let config: ProjectConfig = ProjectConfig {
            name: "PHP".to_string(),
            directory: "/home/user/code/php".to_string(),
            icon: Some(" ".to_string()),
            color: Some("#ff0000".to_string()),
        };
        let project: Project = Project::new(
            "/home/user/code/php/my-project".to_string(),
            &config,
        );

        assert_eq!(Cow::from("[PHP] my-project"), project.session_name());
        assert_eq!(Cow::from("#[fg=#ff0000] #[fg=default] my-project"), project.tmux_display());
        assert_eq!(Cow::from("\x1b[38;2;255;0;0m \x1b[m my-project"), project.skim_display());
        assert_eq!(Cow::from("[PHP] my-project"), project.skim_text());
    }
}
