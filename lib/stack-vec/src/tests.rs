    use crate::StackVec;

    #[test]
    fn assignment_text_example() {
        let mut storage = [0u8; 1024];
        let mut vec = StackVec::new(&mut storage);

        for i in 0..10 {
            vec.push(i * i).expect("can push 1024 times");
        }

        for (i, v) in vec.iter().enumerate() {
            assert_eq!(*v, (i * i) as u8);
        }

        let last_element = vec.pop().expect("has elements");
        assert_eq!(last_element, 9 * 9);
    }

    #[test]
    fn len_and_capacity_ok() {
        let mut storage = [0u8; 1024];
        let stack_vec = StackVec::new(&mut storage);

        assert_eq!(stack_vec.len(), 0);
        assert_eq!(stack_vec.capacity(), 1024);
        assert!(stack_vec.is_empty());
        assert!(!stack_vec.is_full());
    }

    #[test]
    #[should_panic]
    fn index_oob() {
        let mut storage = [0u8; 1024];
        let stack_vec = StackVec::new(&mut storage);
        let _ = stack_vec[0];
    }

    #[test]
    #[should_panic]
    fn index_oob_after_truncate() {
        let mut storage = [0u8; 1024];
        let mut stack_vec = StackVec::new(&mut storage);
        stack_vec.push(10).expect("len > 0");
        stack_vec.truncate(0);
        let _ = stack_vec[0];
    }

    #[test]
    fn indexing() {
        let mut storage = [0u8; 1024];
        let mut stack_vec = StackVec::new(&mut storage);
        assert!(stack_vec.is_empty());

        stack_vec.push(10).expect("cap = 1024");
        assert_eq!(stack_vec[0], 10);
        assert_eq!(stack_vec.len(), 1);
        assert_eq!(stack_vec.capacity(), 1024);
        assert!(!stack_vec.is_empty());

        stack_vec.push(2).expect("cap = 1024");
        assert_eq!(stack_vec[0], 10);
        assert_eq!(stack_vec[1], 2);
        assert_eq!(stack_vec.len(), 2);
        assert_eq!(stack_vec.capacity(), 1024);

        stack_vec.truncate(0);
        assert!(stack_vec.is_empty());
        assert_eq!(stack_vec.len(), 0);
        assert_eq!(stack_vec.capacity(), 1024);

        for i in 0..100 {
            stack_vec.push(i).expect("cap = 1024");
        }

        assert_eq!(stack_vec.len(), 100);
        for i in 0..100 {
            assert_eq!(stack_vec[i], i as u8);
        }
    }

    #[test]
    fn mut_indexing() {
        let mut storage = [0u8; 1024];
        let mut stack_vec = StackVec::with_len(&mut storage, 3);

        assert_eq!(stack_vec[0], 0);
        assert_eq!(stack_vec[1], 0);
        assert_eq!(stack_vec[2], 0);

        stack_vec[0] = 100;
        stack_vec[1] = 88;
        stack_vec[2] = 99;

        assert_eq!(stack_vec[0], 100);
        assert_eq!(stack_vec[1], 88);
        assert_eq!(stack_vec[2], 99);

        stack_vec[0] = 23;
        assert_eq!(stack_vec[0], 23);

        stack_vec[0] = stack_vec[1];
        assert_eq!(stack_vec[0], 88);
    }

    #[test]
    fn pop() {
        let mut storage = [0usize; 1024];
        let mut stack_vec = StackVec::new(&mut storage);
        assert!(stack_vec.pop().is_none());

        stack_vec.push(123).expect("cap = 1024");
        assert_eq!(stack_vec.len(), 1);
        assert_eq!(stack_vec.pop(), Some(123));

        for i in 0..1024 {
            assert_eq!(stack_vec.len(), i);
            stack_vec.push(i).expect("cap = 1024");
            assert_eq!(stack_vec.len(), i + 1);
        }

        for i in 1023..=0 {
            assert_eq!(stack_vec.len(), i + 1);
            assert_eq!(stack_vec.pop(), Some(i));
            assert_eq!(stack_vec.len(), i);
        }
    }

    #[test]
    fn push_just_far_enough() {
        let mut storage = [0usize; 2];
        let mut stack_vec = StackVec::new(&mut storage);
        stack_vec.push(1).expect("okay");
        stack_vec.push(2).expect("okay");
        assert!(stack_vec.is_full());
    }

    #[test]
    #[should_panic]
    fn push_too_far() {
        let mut storage = [0usize; 2];
        let mut stack_vec = StackVec::new(&mut storage);
        stack_vec.push(1).expect("okay");
        stack_vec.push(2).expect("okay");
        stack_vec.push(3).expect("not okay");
    }

    #[test]
    fn iterator() {
        let mut storage = [0usize; 1024];
        let mut stack_vec = StackVec::new(&mut storage);
        assert!(stack_vec.iter().next().is_none());

        stack_vec.push(123).expect("cap = 1024");
        assert_eq!(stack_vec.len(), 1);

        for _ in 0..10 {
            let mut iter = stack_vec.iter();
            assert_eq!(iter.next(), Some(&123));
            assert_eq!(iter.next(), None);
        }

        stack_vec.truncate(0);
        assert!(stack_vec.iter().next().is_none());

        for i in 0..1024 {
            stack_vec.push(i * i).expect("cap = 1024");
        }

        for (i, val) in stack_vec.iter().enumerate() {
            assert_eq!(*val, i * i);
        }

        let mut i = 0;
        for val in &stack_vec {
            assert_eq!(*val, i * i);
            i += 1;
        }

        let mut i = 0;
        for val in stack_vec {
            assert_eq!(*val, i * i);
            i += 1;
        }
    }

    #[test]
    fn as_slice() {
        let mut storage = [0usize; 5];
        let mut stack_vec = StackVec::new(&mut storage);
        assert_eq!(stack_vec.as_slice(), &[]);

        stack_vec.push(102).expect("cap = 5");
        assert_eq!(stack_vec.as_slice(), &[102]);
        assert_eq!(stack_vec.as_mut_slice(), &mut [102]);

        stack_vec.push(1).expect("cap = 5");
        assert_eq!(stack_vec.as_slice(), &[102, 1]);
        assert_eq!(stack_vec.as_mut_slice(), &mut [102, 1]);

        assert_eq!(stack_vec.pop(), Some(1));
        assert_eq!(stack_vec.as_slice(), &[102]);
        assert_eq!(stack_vec.as_mut_slice(), &mut [102]);
    }

    #[test]
    fn errors() {
        let mut storage = [0usize; 1024];
        let mut vec = StackVec::new(&mut storage);
        for i in 0..1024 {
            assert_eq!(vec.push(i), Ok(()));
        }
        for i in 0..1024 {
            assert_eq!(vec.push(i), Err(()));
        }
        for i in 1023..=0 {
            assert_eq!(vec.pop(), Some(i));
        }
        for _ in 1023..=0 {
            assert_eq!(vec.pop(), None);
        }
    }

    // The tests from here on are custom tests for the corner cases

    #[test]
    fn zero_capacity() {
        // Test using an empty backing storage.
        let mut storage: [u8; 0] = [];
        let mut vec = StackVec::new(&mut storage);
    
        // The capacity should be zero, and the vector is empty.
        assert_eq!(vec.capacity(), 0);
        assert!(vec.is_empty());
        // Iteration should yield nothing.
        assert!(vec.iter().next().is_none());
        // Pushing should fail.
        assert_eq!(vec.push(1), Err(()));
        // Popping should return None.
        assert!(vec.pop().is_none());
    }
    
    #[test]
    #[should_panic]
    fn zero_capacity_indexing() {
        // Attempting to index into a zero-capacity vector should panic.
        let mut storage: [u8; 0] = [];
        let vec = StackVec::new(&mut storage);
        let _ = vec[0];
    }
    
    #[test]
    fn push_pop_boundary() {
        // Test pushing exactly up to capacity and then popping all items.
        let mut storage = [0; 3];
        let mut vec = StackVec::new(&mut storage);
    
        // Push three elements (exactly the capacity).
        assert_eq!(vec.push(10), Ok(()));
        assert_eq!(vec.push(20), Ok(()));
        assert_eq!(vec.push(30), Ok(()));
        assert!(vec.is_full());
    
        // A push beyond capacity should return Err.
        assert_eq!(vec.push(40), Err(()));
    
        // Now pop all elements in LIFO order.
        assert_eq!(vec.pop(), Some(30));
        assert_eq!(vec.pop(), Some(20));
        assert_eq!(vec.pop(), Some(10));
    
        // Once empty, further pops return None.
        assert_eq!(vec.pop(), None);
    }
    
    #[test]
    fn with_len_constructor() {
        // Test the `with_len` constructor where the first `len` elements are considered "pushed".
        let mut storage = [0; 5];
        let mut vec = StackVec::with_len(&mut storage, 3);
    
        // The vector starts with length 3.
        assert_eq!(vec.len(), 3);
        // Set values via indexing.
        vec[0] = 100;
        vec[1] = 200;
        vec[2] = 300;
        assert_eq!(vec[0], 100);
        assert_eq!(vec[1], 200);
        assert_eq!(vec[2], 300);
    
        // Popping should remove the last element.
        assert_eq!(vec.pop(), Some(300));
        assert_eq!(vec.len(), 2);
    }
    
    #[test]
    fn truncate_behavior() {
        // Create a vector with 5 elements.
        let mut storage = [1, 2, 3, 4, 5];
        let mut vec = StackVec::with_len(&mut storage, 5);
    
        // Truncating to a value greater than the current length should do nothing.
        vec.truncate(10);
        assert_eq!(vec.len(), 5);
    
        // Truncate to a smaller length.
        vec.truncate(2);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.as_slice(), &[1, 2]);
    
        // Truncating to the same length should leave it unchanged.
        vec.truncate(2);
        assert_eq!(vec.len(), 2);
    }
    
    #[test]
    #[should_panic]
    fn indexing_out_of_bounds() {
        // Indexing beyond the current length should panic.
        let mut storage = [0; 3];
        let mut vec = StackVec::new(&mut storage);
        vec.push(1).unwrap();
        // Only index 0 is valid because len == 1.
        let _ = vec[1];
    }
    
    #[test]
    fn mixed_operations() {
        // A sequence of operations that mix push, pop, and truncate.
        let mut storage = [0; 5];
        let mut vec = StackVec::new(&mut storage);
    
        // Start empty.
        assert_eq!(vec.len(), 0);
    
        // Push three elements.
        vec.push(10).unwrap();
        vec.push(20).unwrap();
        vec.push(30).unwrap();
        assert_eq!(vec.len(), 3);
    
        // Pop one element.
        assert_eq!(vec.pop(), Some(30));
        assert_eq!(vec.len(), 2);
    
        // Truncate to length 1.
        vec.truncate(1);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.as_slice(), &[10]);
    
        // Push two more elements until full.
        vec.push(40).unwrap();
        vec.push(50).unwrap();
        assert_eq!(vec.len(), 3);
        assert!(!vec.is_full()); // Still not full as capacity is 5.
    
        // Verify the current state.
        assert_eq!(vec.as_slice(), &[10, 40, 50]);
    
        // Pop all elements.
        assert_eq!(vec.pop(), Some(50));
        assert_eq!(vec.pop(), Some(40));
        assert_eq!(vec.pop(), Some(10));
        assert_eq!(vec.pop(), None);
    }
    
    #[test]
    fn with_len_preserves_storage() {
        let mut storage = [10, 20, 30, 40, 50];
        let stack_vec = StackVec::with_len(&mut storage, 3);

        assert_eq!(stack_vec.as_slice(), &[10, 20, 30]);
    }

    // #[test]
    // #[should_panic]
    // fn iterator_ownership() {
    //     let mut storage = [0usize; 1024];
    //     let mut stack_vec = StackVec::new(&mut storage);
        
    //     for val in &stack_vec{}
    //     for val in stack_vec{}

    //     let steps = || -> Result<(), MyError> {
    //         for val in stack_vec{}
    //         for val in &stack_vec{}
    //         Ok(())
    //     };

    //     if let Err(_err) = steps() {
    //         panic!("stack_vec not owned!");
    //     }
    // }
