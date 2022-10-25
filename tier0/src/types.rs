use std::rc::Rc;
use std::sync::Arc;

pub type HeapPtr<T> = Arc<Rc<Box<T>>>;

pub fn heap_ptr<T>( value: T ) -> HeapPtr<T> {
	Arc::new( Rc::new( Box::new( value ) ) )
}
