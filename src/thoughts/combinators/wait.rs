/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/

//! # A Thinkable ready after a given amount of time with the given value
//!
use crate::thoughts::{Conclusion, Context, Thinkable};
use core::pin::Pin;
use ruspiro_timer::*;

pub enum WaitThought<T> {
    Initial { delay: Mseconds, value: Option<T> },
    Ready(Option<T>),
}

pub fn wait<T>(delay: Mseconds, value: T) -> WaitThought<T> {
    WaitThought::Initial { 
        delay,
        value: Some(value)
    }
}

impl<T> Thinkable for WaitThought<T> {
    type Output = T;

    fn think(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Conclusion<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        match this {
            WaitThought::Initial{delay, value} => {
                let waker = cx.waker().clone();
                schedule(*delay, move || {
                    waker.wake();
                });

                *this = WaitThought::Ready(value.take());
                Conclusion::Pending
            },
            WaitThought::Ready(value) => Conclusion::Ready(value.take().unwrap()),
        }
    }
}