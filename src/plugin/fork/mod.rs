use amethyst::{
    assets::{PrefabLoader, PrefabLoaderSystemDesc, RonFormat},
    core::transform::TransformBundle,
    ecs::prelude::WorldExt,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::RenderToWindow,
        rendy::{
            hal::command::ClearColor,
            mesh::{Normal, Position, TexCoord},
        },
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, ToNativeWidget, UiBundle, UiCreator, UiTransformData, UiWidget},
    utils::{application_root_dir, scene::BasicScenePrefab},
    window::{DisplayConfig, EventLoop},
};
use skulpin::winit::event_loop::EventLoopWindowTarget;

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;
struct Example;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        // Initialise the scene with an object, a light and a camera.
        let handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/sphere.ron", RonFormat, ())
        });
        world.create_entity().with(handle).build();

        // Load custom UI prefab
        // world.exec(|mut creator: UiCreator<'_, CustomUi>| {
        //     // creator.create("ui/custom.ron", ());
        // });
    }
}

pub struct ForkPlugin {
}

impl ForkPlugin {
    pub fn new(event_loop: EventLoop<()>) {
        let app_root = application_root_dir().unwrap();
        let display_config_path = app_root.join("src/plugin/fork/configs/display.ron");
        let assets_dir = app_root.join("src/plugin/fork/assets");
        let display_config = DisplayConfig::load(display_config_path).unwrap();
        let game_data = GameDataBuilder::default()
            .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
            .with_bundle(TransformBundle::new()).unwrap()
            .with_bundle(InputBundle::<StringBindings>::new()).unwrap()
            // .with_bundle(UiBundle::<StringBindings, CustomUi>::new()).unwrap()
            .with_bundle(
                RenderingBundle::<DefaultBackend>::new(display_config, &event_loop)
                    .with_plugin(RenderToWindow::new().with_clear(ClearColor {
                        float32: [0.34, 0.36, 0.52, 1.0],
                    }))
            ).unwrap();

        let mut game = Application::new(assets_dir, Example, game_data).unwrap();
        game.run_winit_loop(event_loop);
    }
}
