# bevy_resolution
[![Crates.io](https://img.shields.io/crates/v/bevy_resolution)](https://crates.io/crates/bevy_resolution)
[![docs.rs](https://docs.rs/bevy_resolution/badge.svg)](https://docs.rs/bevy_resolution/)
![License](https://img.shields.io/crates/l/bevy_resolution)
![Bevy Version](https://img.shields.io/badge/bevy%20version-0.15.0--rc.3-blue)

 `bevy_resolution` aims to provide a simple, easy, and convenient way to set and manage window resolutions. It provides
conveniences for commonly-supported Window Resolutions *and* also provides a way to manage window resolution scaling for
arbitrary resolutions.

## Features
- Provides an easy, but powerful, API for resolution management
- Provides convenience wrappers for common 16:9 and 4:3 resolutions
- Can support any resolution with any Aspect Ratio and uses Bevy's built-in `AspectRatio`

## Feature Flags
This crate exposes the following feature flags:  
- `bevy_window`: Enables support for usage with `bevy_window`, this is required for usage with `bevy_window::WindowResolution`

The `bevy_window` feature is enabled by default.

## Supported Bevy Versions
| Bevy Resolution Version | Bevy Version |
|:-----------------------:|:------------:|
|          0.1.0          |     0.15     |

## Acknowledgements  
I just want to thank the people who gave feedback on the initial Issue/PR even if it didn't make it in. That feedback
did shape and affect the overall design of this crate.

## QnA
### Why does this package exist?  
This actually comes from Bevy PR #14158, which attempted to add in a set of commonly used resolutions for developers
to use. This was, unfortunately, rejected. While I initially did not intend to make this as a package, I came up with
some ideas on this and decided to make it.

### What is an Approximate Resolution?  
An approximate resolution is a resolution that, for a given Aspect Ratio, is not *exactly* that Aspect Ratio, but
approximately that (IE ~=16:9). This is because some resolutions can not evenly support an integer height and width for
a given ratio and will instead have a decimal point. If this is the case, it's an Approximate Resolution. 

A resolution that has an integer height and width at a given Aspect Ratio (and as such is *exactly* that Aspect Ratio)
is an Exact Resolution.

### What's the deal with `CommonResolutions`?  
`CommonResolutions` is there to allow you to more easily reach for and use resolutions you may want. Admittedly this is
the most opinionated part of this crate, as it provides only 16:9 and 4:3 resolutions for quick use. This is open to
change in the future, however the resolutions I provided are what I tend to use. The 4:3 resolutions are added as
someone might want to use the 4:3 variants instead.

### Why does 1080p not appear in the 4:3 `CommonResolutions`?  
Alright, so this is actually a rather interesting thing. All the resolutions listed -- except for 4:3 -- have a
4:3 standard...1080p doesn't have that (at least according to Wikipedia and a bit of googling). This does not mean that 
4:3 1080p isn't an exact resolution, it is. However, it's not really expected, and as such I didn't include it. 

This doesn't mean that I'm against including it, I just felt it was best to not do so at this time.

### Does this support Ultra-Widescreen?  
We can support any given Resolution and Aspect Ratio. However, we don't provide then through `CommonResolutions`, for 
now.

### Is this really that useful?  
Yes, I use a similar pattern when I'm working on games (particularly 2D Pixel-Art games).

### Does this really need to exist?  
Maybe, Maybe not. However, in every project I make, I tend to end up with a list of supported resolutions, and having
to manage and remember this each time is a bit of a pain. I also figured others could probably use this. This is a bit
different from #14158, in that it's a bit more powerful in a few ways -- such as being able to do dynamic resolutions.

### How do you handle non-exact/approximate resolutions? (IE: 16:9 480p)?  
For approximate resolutions (IE: resolutions that don't fit exactly into an aspect ratio), they are represented as their
exact value, if it's a decimal. However, when converting to `WindowResolution` (if enabled) or to `UVec2` the resolution
will always round that number up. This is because, it's arguably the best way to ensure that nothing is accidentally
cut off. As an example if you want 480p in 16:9, it will be 854x480 instead of 853.3333x480