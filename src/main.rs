use std::time::Duration;
use bevy::{prelude::*, window::PrimaryWindow, render::camera::SubCameraView, time::common_conditions::on_timer};
use once_cell::sync::Lazy;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use wasm_bindgen::prelude::wasm_bindgen;

const WINDOW_WIDTH: f32 = 500.;
const WINDOW_HEIGHT: f32 = 500.;
const DISPLAY_FULL_SIZE: Lazy<UVec2> = Lazy::new(|| UVec2::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32));
const DISPLAY_SIZE: Lazy<UVec2> = Lazy::new(|| UVec2::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32));
const SNAKE_HEAD_COLOR: Srgba = Srgba::rgb(0.7, 0.7, 0.7);
const FOOD_COLOR: Srgba = Srgba::rgb(1.0, 0.0, 1.0);
const SNAKE_SEGMENT_COLOR:  Srgba = Srgba::rgb(0.3, 0.3, 0.3);
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[wasm_bindgen]
extern "C" {
    fn get_seed() -> u32;
}

// ヘビの頭
#[derive(Component)]
struct SnakeHead {
    direction: Direction
}

// フード
#[derive(Component)]
struct Food;

// へびのしっぽ
#[derive(Component)]
struct SnakeSegment;

// へびのしっぽ（リスト）
#[derive(Default, Resource)]
struct SnakeSegments(Vec<Entity>);

// ヘビの位置
#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

// ヘビのサイズ
#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

// ヘビの方向
#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

// ヘビがフードを食べたイベント
#[derive(Event)]
struct GrowthEvent;

// しっぽの最後尾を保持するリソース
#[derive(Default, Resource)]
struct LastTailPosition(Option<Position>);

// ゲームオーバーイベント
#[derive(Event)]
struct GameOverEvent;

fn setup_camera(mut commands: Commands) {
    let bundle = (
        Camera2d,
        Camera {
            sub_camera_view: Some(SubCameraView {
                full_size: *DISPLAY_FULL_SIZE,
                offset: Vec2::new(0.0, 0.0),
                size: *DISPLAY_SIZE,
            }),
        order: 1,
        ..default()
        }
    );
 
    commands.spawn(bundle);
}

//
// ヘビの頭を出現させる
//
fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    let bundle = (
        Sprite::from_color(SNAKE_HEAD_COLOR, Vec2::new(1.0, 1.0)),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 0.0),
            ..default()
        },
        Position { x: 4, y: 5 },
        Size::square(0.8),
        SnakeHead {
            direction: Direction::Up
        }
    );

    // ２つのエンティティをリソースに追加したSnakeSegmentsのVec<Entity>に追加
    * segments = SnakeSegments(vec![
        commands.spawn(bundle).id(),
        spawn_segment(commands, Position { x: 4, y: 4 }),
    ])
}

//
// フードを出現させる
//
fn spawn_food(mut commands: Commands) {
    #[allow(unused_unsafe)]
    let rng = SmallRng::seed_from_u64( unsafe { get_seed() } as u64 );

    let arena_width = ARENA_WIDTH as i32;
    let arena_height = ARENA_HEIGHT as i32;
    let bundle = (
        Sprite::from_color(FOOD_COLOR, Vec2::new(1.0, 1.0)),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 0.0),
            ..default()
        },
        Position {
            x: rng.clone().random_range(0..arena_width),
            y: rng.clone().random_range(0..arena_height),
        },
        Size::square(0.8),
        Food
    );

    commands.spawn(bundle);
}

// ヘビのしっぽを出現させる
fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    let bundle = (
        Sprite::from_color(SNAKE_SEGMENT_COLOR, Vec2::new(1.0, 1.0)),
        SnakeSegment,
        Size::square(0.65)
    );
    commands.spawn(bundle)
            .insert(position)
            .id()
}

//
// ヘビの頭を動かす方向の入力を受け取る
//
fn snake_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut heads: Query<&mut SnakeHead>
) {
    if let Some(mut head) = heads.iter_mut().next() {
        let direction: Direction = if keyboard_input.pressed(KeyCode::ArrowLeft) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::ArrowUp) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            Direction::Right
        } else {
            head.direction
        };

        // 入力された方向がヘビの進行方向の反対方向でなければ方向転換
        if direction != head.direction.opposite() {
            head.direction = direction;
        }
    }
}

//
// ヘビの頭を動かす
//
fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>, // ←
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>
){
    if let Some((head_entity, head)) = heads.iter_mut().next() {
         let segment_positions = segments.0.iter()
                                         .map(|e| *positions.get_mut(*e).unwrap())
                                         .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };

        // ヘビの頭の位置が壁を越えていたらゲームーオーバーイベントをキューに追加
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_WIDTH
            || head_pos.y as u32 >= ARENA_HEIGHT
        {
            game_over_writer.write(GameOverEvent);
        }

        // ヘビの頭がしっぽの位置に含まれた場合もゲームオーバーイベントをキューに追加
        if segment_positions.contains(&head_pos) {
            game_over_writer.write(GameOverEvent);
        }

        segment_positions.iter().zip(segments.0.iter().skip(1)).for_each(|(pos, segment)| {
            *positions.get_mut(*segment).unwrap() = *pos;
        });

        // ここでしっぽの末尾の位置をリソースに保持させる
        *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
    }
}

//
// ヘビがフードを食べる
//
fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.write(GrowthEvent);
            }
        }
    }
}

//
// ヘビが成長する
//
fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if let Some(_event) = growth_reader.read().next() {
        segments.0.push(spawn_segment(commands, last_tail_position.0.unwrap()));
    }
}

//
// ゲームーオーバー
//
fn game_over(
    mut commands: Commands,
    mut game_over_reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    heads: Query<Entity, With<SnakeHead>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if let Some(_event) = game_over_reader.read().next() {
        for ent in food.iter().chain(segments.iter()).chain(heads.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segments_res);
    }
}

//
// 画面に表示されるモノの大きさをスケールする
//
fn size_scaling(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>
) {
    if let Some(window) = windows.iter_mut().next() {
        for (sprite_size, mut transform) in q.iter_mut() {
            transform.scale = Vec3::new(
                sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
                sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
                1.0,
            );
        }
    }
}

//
// スケールに合わせて表示位置を更新する
//
fn position_translation(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform)>
) {
    if let Some(window) = windows.iter_mut().next() {
        fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
            let tile_size = bound_window / bound_game;
            (pos / bound_game * bound_window) - (bound_window / 2.) + (tile_size / 2.)
        }

        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
                convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
                0.0,
            );
        }
    }
}


fn main() {
    App::new()
         .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
         .insert_resource(SnakeSegments::default())
         .insert_resource(LastTailPosition::default())
         .add_event::<GrowthEvent>()
         .add_event::<GameOverEvent>() // ←
         .add_plugins(DefaultPlugins.set( WindowPlugin {
            primary_window: Some( Window {
                title: "Snake!".into(),
                name: Some("Snake.app".into()),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
         }))
         .add_systems(Startup, setup_camera)
         .add_systems(Startup, spawn_snake)
         .add_systems(Update, snake_movement.run_if(on_timer(Duration::from_millis(500)))) // ← へびの動きをもう少し速く
         .add_systems(Update, snake_movement_input.before(snake_movement))
         .add_systems(Update, spawn_food.run_if(on_timer(Duration::from_secs(3)))) // フードの出現量を抑える
         .add_systems(Update, snake_eating.after(snake_movement))
         .add_systems(Update, snake_growth.after(snake_eating))
         .add_systems(Update, game_over.after(snake_movement))
         .add_systems(PostUpdate, position_translation)
         .add_systems(PostUpdate, size_scaling)
         .run()
         ;
}
