use hashbrown::HashMap;
use spin::{Mutex};
use crate::std::render::{Frame, RenderError};

use alloc::{
	boxed::Box,
	sync::Arc,
	string::String,
};
use core::any::Any;
use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::std::application::Exit;
use crate::std::io::KeyStroke;

/// implement this trait if you require the widget to be able to have an outline
pub trait CgOutline: CgComponent {
	fn render_outline(&self, frame: &mut Frame);
}

/// generic components for the user interface that defined a render method. this should be implemented for all types
/// that can be rendered to the screen.
pub trait CgComponent: Any {
	fn render(&self) -> Result<Frame, RenderError>;

	fn as_any(&self) -> &dyn Any;
}

/// trait for components that can have editable text, such as search boxes, command palettes, terminals, text inputs etc.
pub trait CgTextEdit: CgComponent {
	fn write_char(&mut self, c: char); // this can also be implemented in a way that inserts characters
	fn backspace(&mut self);
	fn move_cursor(&mut self, direction: bool); // true = right, false = left
	fn clear(&mut self);
}

#[async_trait]
pub trait CgTextInput: CgTextEdit {
	async fn input(&mut self, break_condition: fn(KeyStroke) -> (KeyStroke, Exit), id: &Widget, app: &Widget) -> Result<(String, bool), RenderError>;
}

#[async_trait]
pub trait CgKeyboardCapture: CgComponent {
	async fn keyboard_capture(&mut self, break_condition: fn(KeyStroke) -> (KeyStroke, Exit), app: Option<&Widget>) -> Result<(Exit, usize), RenderError>;
}

static ID_COUNTER: Mutex<usize> = Mutex::new(0);

lazy_static!(
	static ref GUITREE: Mutex<DataStore> = Mutex::new(DataStore::new());
);



#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Widget {
	id: usize,
}
impl Widget {
	fn new() -> Self {
		let mut id_counter = ID_COUNTER.lock();
		let id = Widget {
			id: *id_counter
		};
		*id_counter += 1;
		id
	}

	pub fn fetch<T: 'static + Send + Sync + Clone + CgComponent>(&self) -> Option<T> {
		GUITREE.lock().fetch(self)
	}

	pub fn insert<T: 'static + Send + Sync + Clone + CgComponent>(item: T) -> Self {
		let mut id_counter = ID_COUNTER.lock();
		let id = Widget { id: *id_counter };
		*id_counter += 1;
		GUITREE.lock().insert(&id, item);
		id
	}

	pub fn update<T: 'static + Send + Sync + Clone + CgComponent>(&self, item: T) {
		GUITREE.lock().insert(self, item);
	}

	pub fn render(&self) -> Result<Frame, RenderError> {
		let component_arc_mutex = match GUITREE.lock().frame(self) {
			Some(component) => component,
			None => panic!("CRITICAL: Widget not found in tree"),
		};

		let component = component_arc_mutex.lock();
		component.render()
	}
}

impl Drop for Widget {
	fn drop(&mut self) {
		let removed = GUITREE.lock().remove(self);
		drop(removed);
	}
}



struct DataStore {
	items: Mutex<HashMap<usize, Arc<Mutex<dyn CgComponent + Send + Sync>>>>,
	id_counter: Mutex<usize>,
}

impl DataStore {
	fn new() -> Self {
		DataStore {
			items: Mutex::new(HashMap::new()),
			id_counter: Mutex::new(0),
		}
	}

	fn insert<T: 'static + CgComponent + Send + Sync + Clone>(&self, id: &Widget, item: T) {
		let mut items = self.items.lock();
		items.insert(id.id, Arc::new(Mutex::new(item)));
	}

	fn fetch<T: 'static + Send + Sync + Clone>(&self, id: &Widget) -> Option<T> where T: Any + Send + Sync {
		let id = id.id;

		let items = self.items.lock();
		items.get(&id).and_then(|arc| {
			let any_mutex = arc.lock();
			let any_ref = any_mutex.as_any();
			any_ref.downcast_ref::<T>().cloned()
		})
	}

	fn frame(&self, id: &Widget) -> Option<Arc<Mutex<dyn CgComponent + Send + Sync + 'static>>> {
		let items = self.items.lock();
		items.get(&id.id).cloned()
	}

	fn remove(&self, id: &Widget) -> Option<Arc<Mutex<dyn CgComponent + Send + Sync>>> {
		let mut items = self.items.lock();
		items.remove(&id.id)
	}
}





