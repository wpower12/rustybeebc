# 2021-11-12 - Starting Up.

Lets remember how this all works. 

So we'll need to get our bearings with types, structs, etc again. Then parse the modules from the js source code into the main, and just get those all created and initialized. 

## EBC Struct
Ok so we need to think like rustcrabs. What do we have to do to make the registers and such mutable, and still 'safe'? I guess I'll just try to write and modify and follow the errors till I know what to ask the google. 

Got stuck on const/enums/etc. Ended up just using consts. I'd like to learn a more "rusty" way of handling all the flags. Still want to be able to boolean on them easily. 
