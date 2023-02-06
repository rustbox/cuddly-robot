# Keyboard Stretegy Discussion

## 1. Use a PS/2 compatible keyboard, and implement PS/2 standard in hardware/software

### PRO:

Finding a compatible keyboard would likely be easy.

Implementing PS/2 does not seem very hard (not having done it yet lol)

Would get us going with a keyboard relatively quickly

### CON:

Aesthetically, it's a bit of a bummer to have to restrict the types of keyboards we coould accept.

### To Consider:

PS/2, while probably not hard, is likely not trivial

## 2. Use a sparkfun breakout board

### PRO:

Fast solution

### Con:

Expensive

### To Consider:

Less "fun" to make as we would hopefully just have to plug a few things together

## 3. Get a IC that implements the USB HID Host and integrate it into the project

### PRO:

Feels like it nails the aesthetic of the project

### Con:

Could become slow to implement or involved, unknown real difficulty

These ICs seem to all be surface mount which are more difficult to solder

"Expensive" for an IC. With thinking of this as a "product" or reproducible, it's a bit of a bummer to have to require expensive parts, making it harder for someone to duplicate our work. (Eric is likely overthinking this for the keyboard specifically - we have many "expensive" chips in this project)

## 4. Implement software USB host, perhaps using existing code

### Pro:

Also hits the aethetic of the project Eric thinks.

Depending on how exactly we integrate this into the rest of the project, may not be that hard

### Con:

Very unknown difficulty, could be extremely fraught, especially if we wanted to implement the host on the same device as the one we're using for outputting the pixel data to the screen

"Expensive" to just use a 2nd esp32, if we did that

### To Consider:

It seems to me the easist way to do this is to use a seperate esp32c3 so as to play well the best with the software that implements the soft USB host.

## 5. Fake it by using bluetooth instead

### Pro:

Would work

### Con:

Seems silly - doesn't really match the aesthetic

How hard is turning on bluetooth on the CPU?

### To consider:

Would this only allow bluetooth keyboards that are battary operated to function then? If so, also seems bad.
