use bevy::prelude::*;

pub fn create_app() -> App {
    let mut app = App::new();

    // The function 'try_add_plugins' 
    // (https://github.com/bevyengine/bevy/discussions/15802#discussioncomment-10898148)
    // will make this if obsolete and increase code coverage.
    // Thanks mgi388 for pointing this out
    if cfg!(test) {
        app.add_plugins(bevy::window::WindowPlugin::default());
    } else {
        app.add_plugins(DefaultPlugins);
    }
    assert!(app.is_plugin_added::<bevy::window::WindowPlugin>());
    set_window_title(&mut app, String::from("My favorite title"));

    app
}

fn count_n_windows(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Window>();
    query.iter(app.world()).len()
}

#[cfg(test)]
fn get_window_title(app: &mut App) -> String {
    assert!(app.is_plugin_added::<bevy::window::WindowPlugin>());
    assert_eq!(count_n_windows(app), 1);
    let mut query = app.world_mut().query::<&Window>();
    let window = query.single(app.world());
    window.title.clone()
}

fn set_window_title(app: &mut App, title: String) {
    assert!(app.is_plugin_added::<bevy::window::WindowPlugin>());
    assert_eq!(count_n_windows(app), 1);
    let mut query = app.world_mut().query::<&mut Window>();
    let mut window = query.single_mut(app.world_mut());
    window.title = title;

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_an_empty_app_has_no_windows() {
        let mut app = App::new();
        app.update();
        assert_eq!(count_n_windows(&mut app), 0);
    }

    #[test]
    fn test_our_app_has_one_window() {
        let mut app = create_app();
        app.update();
        assert_eq!(crate::app::count_n_windows(&mut app), 1);
    }

    #[test]
    fn test_app_has_our_window_title() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_window_title(&mut app), "My favorite title");
    }

    #[test]
    fn test_app_has_any_window_title() {
        let mut app = create_app();
        let title = String::from("Any title");
        set_window_title(&mut app, title.clone());
        app.update();
        assert_eq!(get_window_title(&mut app), title);
    }
}
