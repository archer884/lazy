Lazy
====

You could also call this, like, the "Lazy T" or something, which sounds like the name of a dude ranch just outside Los Angeles or some shit. Next library I write is going to be some super-generic thing called Bar T. I'm going to take this opportunity seriously, gentlemen.

This (uber) simple library allows you to create lazily initialized objects: items that don't have a value until the value is called for so that, I guess, you can shave like 0.00000001 milliseconds off your runtime in the event the value is never actually required. I don't know why this exists. I use the lazy initialization pattern quite a lot, actually, but I have never used .NET's Lazy<T> even one time in my entire career.

That's life, I guess.
