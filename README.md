# Bevy Resolutions
 Bevy Resolutions aims to provide a simple, easy, and convenient way to set and manage window resolutions. It provides
conveniences for commonly-supported Window Resolutions *and* also provides a way to manage window resolution scaling for
arbitrary resolutions.

## Examples
TODO

## QnA
### Why does this package exist?  
This actually comes from Bevy PR #14158, which attempted to add in a set of commonly used resolutions for developers
to use. This was, unfortunately, rejected. While I initially did not intend to make this as a package, I came up with
some ideas on this and decided to make it.

### Why do Resolution and Common Resolution exist?  
Resolution allows you to set a custom aspect ratio and custom resolution, where-as `CommonResolutions` exists as a 
way to make it easier for someone who is new to just set a resolution (or resolutions) and not worry about things.

### Does this support Ultra-Widescreen?  
Yes-ish. You can use an Ultra-Widescreen Resolution, however we don't provide convenience wrappers for them for now.

### Is this really that useful?  
Yes, I use a similar pattern when I'm working on games (particularly 2D Pixel-Art games).

### When should I use `Resolution` over `CommonResolutions`?  
You should use `Resolution` when you need resolutions that are not provided by `CommonResolutions`, OR you want to use
a different aspect ratio (IE: 4:3 1080p, 16:9 480p, etc.)

### Does this really need to exist?  
Maybe, Maybe not. However, in every project I make, I tend to end up with a list of supported resolutions, and having
to manage and remember this each time is a bit of a pain. I also figured others could probably use this. This is a bit
different from #14158, in that it's a bit more powerful in a few ways -- such as being able to do dynamic resolutions.

### How do you handle non-perfect resolutions? (IE: 16:9 480p)?  
We do support them, *however* it will be rounded up to the nearest pixel, so it will be *slightly* larger. This only
applies if you get a UVec2 of the Resolution. However, if you get a Vec2 it will return the non-rounded value.