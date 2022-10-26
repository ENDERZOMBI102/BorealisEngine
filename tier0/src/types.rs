use std::sync::Arc;

pub type HeapPtr<T> = Arc<Box<T>>;

pub fn heap_ptr<T>( value: T ) -> HeapPtr<T> {
	Arc::new( Box::new( value ) )
}
