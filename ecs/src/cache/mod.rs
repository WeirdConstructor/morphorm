use std::collections::HashMap;

use crate::Entity;

use morphorm::{Cache, GeometryChanged};

#[derive(Debug, Default, Clone, Copy)]
pub struct Rect {
    pub posx: f32,
    pub posy: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Space {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Default)]
pub struct NodeCache {
    // Computed Outputs
    pub rect: HashMap<Entity, Rect>,

    // Intermediate Values
    space: HashMap<Entity, Space>,
    size: HashMap<Entity, Size>,

    child_width_max: HashMap<Entity, f32>,
    child_height_max: HashMap<Entity, f32>,
    child_width_sum: HashMap<Entity, f32>,
    child_height_sum: HashMap<Entity, f32>,

    grid_row_max: HashMap<Entity, f32>,
    grid_col_max: HashMap<Entity, f32>,

    horizontal_free_space: HashMap<Entity, f32>,
    horizontal_stretch_sum: HashMap<Entity, f32>,

    vertical_free_space: HashMap<Entity, f32>,
    vertical_stretch_sum: HashMap<Entity, f32>,

    stack_first_child: HashMap<Entity, bool>,
    stack_last_child: HashMap<Entity, bool>,

    geometry_changed: HashMap<Entity, GeometryChanged>,

    visible: HashMap<Entity, bool>,

    pub layer: HashMap<Entity, usize>,
}

impl NodeCache {
    pub fn add(&mut self, entity: Entity) {
        self.rect.insert(entity, Default::default());

        self.space.insert(entity, Default::default());

        self.child_width_max.insert(entity, Default::default());
        self.child_height_max.insert(entity, Default::default());
        self.child_width_sum.insert(entity, Default::default());
        self.child_height_sum.insert(entity, Default::default());

        self.grid_row_max.insert(entity, Default::default());
        self.grid_col_max.insert(entity, Default::default());

        self.horizontal_free_space
            .insert(entity, Default::default());
        self.horizontal_stretch_sum
            .insert(entity, Default::default());

        self.vertical_free_space.insert(entity, Default::default());
        self.vertical_stretch_sum.insert(entity, Default::default());

        self.stack_first_child.insert(entity, Default::default());
        self.stack_last_child.insert(entity, Default::default());

        self.size.insert(entity, Default::default());

        self.geometry_changed.insert(entity, Default::default());

        self.visible.insert(entity, true);
    }
}

impl Cache for NodeCache {
    type Item = Entity;

    fn visible(&self, node: Self::Item) -> bool {
        if let Some(value) = self.visible.get(&node) {
            return *value;
        }

        true
    }

    fn geometry_changed(&self, node: Self::Item) -> GeometryChanged {
        if let Some(geometry_changed) = self.geometry_changed.get(&node) {
            return *geometry_changed;
        }

        GeometryChanged::default()
    }

    fn set_geo_changed(&mut self, node: Self::Item, flag: GeometryChanged, value: bool) {
        if let Some(geometry_changed) = self.geometry_changed.get_mut(&node) {
            geometry_changed.set(flag, value);
        }
    }

    fn width(&self, node: Self::Item) -> f32 {
        if let Some(rect) = self.rect.get(&node) {
            return rect.width;
        }

        0.0
    }

    fn height(&self, node: Self::Item) -> f32 {
        if let Some(rect) = self.rect.get(&node) {
            return rect.height;
        }

        0.0
    }

    fn posx(&self, node: Self::Item) -> f32 {
        if let Some(rect) = self.rect.get(&node) {
            return rect.posx;
        }

        0.0
    }

    fn posy(&self, node: Self::Item) -> f32 {
        if let Some(rect) = self.rect.get(&node) {
            return rect.posy;
        }

        0.0
    }

    fn left(&self, node: Self::Item) -> f32 {
        if let Some(space) = self.space.get(&node) {
            return space.left;
        }

        0.0
    }

    fn right(&self, node: Self::Item) -> f32 {
        if let Some(space) = self.space.get(&node) {
            return space.right;
        }

        0.0
    }

    fn top(&self, node: Self::Item) -> f32 {
        if let Some(space) = self.space.get(&node) {
            return space.top;
        }

        0.0
    }

    fn bottom(&self, node: Self::Item) -> f32 {
        if let Some(space) = self.space.get(&node) {
            return space.bottom;
        }

        0.0
    }

    fn new_width(&self, node: Self::Item) -> f32 {
        if let Some(size) = self.size.get(&node) {
            return size.width;
        }

        0.0
    }

    fn new_height(&self, node: Self::Item) -> f32 {
        if let Some(size) = self.size.get(&node) {
            return size.height;
        }

        0.0
    }

    fn child_width_max(&self, node: Self::Item) -> f32 {
        *self.child_width_max.get(&node).unwrap()
    }

    /// Get the computed sum of the widths of the child nodes
    fn child_width_sum(&self, node: Self::Item) -> f32 {
        *self.child_width_sum.get(&node).unwrap()
    }

    /// Get the computed maximum width of the child nodes
    fn child_height_max(&self, node: Self::Item) -> f32 {
        *self.child_height_max.get(&node).unwrap()
    }

    /// Get the computed sum of the widths of the child nodes
    fn child_height_sum(&self, node: Self::Item) -> f32 {
        *self.child_height_sum.get(&node).unwrap()
    }

    /// Get the computed maximum grid row
    fn grid_row_max(&self, node: Self::Item) -> f32 {
        *self.grid_row_max.get(&node).unwrap()
    }

    /// Get the computed maximum grid column
    fn grid_col_max(&self, node: Self::Item) -> f32 {
        *self.grid_col_max.get(&node).unwrap()
    }

    // Setters
    fn set_visible(&mut self, node: Self::Item, value: bool) {
        *self.visible.get_mut(&node).unwrap() = value;
    }

    fn set_child_width_sum(&mut self, node: Self::Item, value: f32) {
        *self.child_width_sum.get_mut(&node).unwrap() = value;
    }

    fn set_child_height_sum(&mut self, node: Self::Item, value: f32) {
        *self.child_height_sum.get_mut(&node).unwrap() = value;
    }

    fn set_child_width_max(&mut self, node: Self::Item, value: f32) {
        *self.child_width_max.get_mut(&node).unwrap() = value;
    }

    fn set_child_height_max(&mut self, node: Self::Item, value: f32) {
        *self.child_height_max.get_mut(&node).unwrap() = value;
    }

    fn horizontal_free_space(&self, node: Self::Item) -> f32 {
        *self.horizontal_free_space.get(&node).unwrap()
    }
    fn set_horizontal_free_space(&mut self, node: Self::Item, value: f32) {
        *self.horizontal_free_space.get_mut(&node).unwrap() = value;
    }
    fn vertical_free_space(&self, node: Self::Item) -> f32 {
        *self.vertical_free_space.get(&node).unwrap()
    }
    fn set_vertical_free_space(&mut self, node: Self::Item, value: f32) {
        *self.vertical_free_space.get_mut(&node).unwrap() = value;
    }

    fn horizontal_stretch_sum(&self, node: Self::Item) -> f32 {
        *self.horizontal_stretch_sum.get(&node).unwrap()
    }
    fn set_horizontal_stretch_sum(&mut self, node: Self::Item, value: f32) {
        *self.horizontal_stretch_sum.get_mut(&node).unwrap() = value;
    }
    fn vertical_stretch_sum(&self, node: Self::Item) -> f32 {
        *self.vertical_stretch_sum.get(&node).unwrap()
    }
    fn set_vertical_stretch_sum(&mut self, node: Self::Item, value: f32) {
        *self.vertical_stretch_sum.get_mut(&node).unwrap() = value;
    }

    fn set_width(&mut self, node: Self::Item, value: f32) {
        if let Some(rect) = self.rect.get_mut(&node) {
            rect.width = value;
        }
    }
    fn set_height(&mut self, node: Self::Item, value: f32) {
        if let Some(rect) = self.rect.get_mut(&node) {
            rect.height = value;
        }
    }
    fn set_posx(&mut self, node: Self::Item, value: f32) {
        if let Some(rect) = self.rect.get_mut(&node) {
            rect.posx = value;
        }
    }
    fn set_posy(&mut self, node: Self::Item, value: f32) {
        if let Some(rect) = self.rect.get_mut(&node) {
            rect.posy = value;
        }
    }

    fn set_left(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(&node) {
            space.left = value;
        }
    }

    fn set_right(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(&node) {
            space.right = value;
        }
    }

    fn set_top(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(&node) {
            space.top = value;
        }
    }

    fn set_bottom(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(&node) {
            space.bottom = value;
        }
    }

    fn set_new_width(&mut self, node: Self::Item, value: f32) {
        if let Some(size) = self.size.get_mut(&node) {
            size.width = value;
        }
    }

    fn set_new_height(&mut self, node: Self::Item, value: f32) {
        if let Some(size) = self.size.get_mut(&node) {
            size.height = value;
        }
    }

    fn stack_first_child(&self, node: Self::Item) -> bool {
        *self.stack_first_child.get(&node).unwrap()
    }

    fn set_stack_first_child(&mut self, node: Self::Item, value: bool) {
        *self.stack_first_child.get_mut(&node).unwrap() = value;
    }

    fn stack_last_child(&self, node: Self::Item) -> bool {
        *self.stack_last_child.get(&node).unwrap()
    }

    fn set_stack_last_child(&mut self, node: Self::Item, value: bool) {
        *self.stack_last_child.get_mut(&node).unwrap() = value;
    }
}