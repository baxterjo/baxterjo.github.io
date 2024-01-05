
#### ABOUT THE PROJECT

This project was the culmination of several miniature projects completed over the course of a class called
"System Design with Microcontrollers." A class which is infamous among OSU students and alumni as being one of the most difficult classes in the ECE program.
The final device is an FM alarm clock with local and remote temperature sensors all built by hand!

#### SKILLS USED

|   |  |
|---|---|
|Embedded System Design| C Programming Language|
| Make | Linux Operating System |

#### THE BRAINS OF THE OPERATION

![](/img/projects/hardware/alarm_clock/clockboard2.jpg)

For this project, the bulk of the computing and I/O power lay in this custom ATMega-128 microcontroller breakout board. This is a standard lab board given to all ECE students at some point in their journey at Oregon State. Some saw it first in lower level electronics classes, but I received mine for the first time in assembly language and computer architecture. The only other µC used in this project is for a remote temperature sensor, but more on that later.

All development done for this project was in a Linux environment. I am a native Windows user, so this meant heavy use of a virtual machine to get the job done.

#### START SIMPLE

![](/img/projects/hardware/alarm_clock/clockdispfront.jpg)

The first mini-project for this class was simple, set up the tool chain. This may seem trivial to many. But since this was my first time developing in a Linux environment, I had to hit the ground running to keep up.

After that we had another seemingly small task: get the 7-segment display and buttons working. The LED display on the front works using a multiplexer and persistance of vision. Meaning only one of the digits is on at any given time. This required the use of a timer interrupt to ensure the correct duty cycle. This same timer was also used to manually debounce the buttons. This was done by checking the buttons once every 2ms and looking for a certain number of active states in a row.

#### BUILD UPON SUCCESS

![](/img/projects/hardware/alarm_clock/clockdispback.jpg)

After the display and button debouncing were complete, we were to add the rotary encoders and light bar to the display, these were interfaced to the µC via SPI. Now, I only had to write to the light bar and read from the encoders. This meant that I could perform the operation simultaneously, by writing to both devices simultaneously and only connecting the encoders to the MISO net, this meant that I was only performing one SPI operation to perform two tasks!
The second large problem of this mini-project was developing a finite state machine (FSM) for the rotary encoders. The ATMega-128 is a fine piece of hardware with great documentation, and highly reliable architecture. But it lacks certain quality of life features that you might see in a more modern µC. This meant defining my own FSM for the encoders. This FSM tracks the past state of the encoders and compares it to the current state. Also tracked is their assumed direction based on the starting position compared to the first change seen. The resulting FSM was very robust and ensured the encoder actions were only performed on a solid 'click' in either direction.

#### STARTING TO LOOK LIKE A CLOCK<br>(DEFINITELY DOESN'T LOOK LIKE A BOMB)

![](/img/projects/hardware/alarm_clock/clockperiphboard.jpg)

Up until this point, my clock didn't keep time. This portion of the project remedies that and then some! This mini-project saw the addition of time setting, time keeping, and alarm setting. But what is the point of setting an alarm unless you know when it's going off? Oh yea, this mini-project also had me building a voltage no pre-amp for the system audio amplifier. This was needed because this clock was going to have alarm and radio functions. The alarm had a range from 0V-5V, and the radio a range of ±250mV. These signals had to be normalized to ensure proper power was being sent to the speakers. This portion of the clock was by-far the most challenging part of the project.

#### FANCY FEATURES

![](/img/projects/hardware/alarm_clock/clockremtemp.jpg)

Here is where the second µC comes into the project. This is a remote temperature sensor that is operated by an arduino nano. This class is very anti-easy though, so our first task was to wipe the arduino bootloader off of the µC and program it from scratch. This part of the system uses I2C to get the temperature from the connected temperature sensor and send it via UART/RS-232 transceiver that sends the data to the main system. The main system board has a mirrored transceiver on its end, so the whole process "looks" like UART to both µCs.

The use of the nano was new this year compared to previous years, and the I2C temperature sensor was new as well. So, the instructor and TAs had no experience assisting students with it. Luckily, I had access to a Saleae logic analyser and was able to quickly troubleshoot some of the interfacing problems that were common on this half of the system. As a result, I was able to quickly write up a "best practices" report to share with my peers, resulting in the class progressing in a timely manner.

#### MUSIC TO MY EARS

![](/img/projects/hardware/alarm_clock/clock2.jpg)

After implementing the remote temperature sensor, I had to return to audio. This portion of the project had me implementing the final piece of hardware into the system, an FM radio receiver. This particular radio receiver (I won't name names) had one of the poorer datasheets I have seen in my limited time in embedded systems. The IC was likely made without a field applications or test engineer on the team. This reinforced my belief that a piece of technology can be beautifully made, but if you cannot communicate its usefulness or how to use it. The technology will not succeed.
But, after much frustration and troubleshooting, the radio worked! And I have never been more happy to hear country music.

#### A (HIDEOUSLY) WONDERFUL CLOCK

![](/img/projects/hardware/alarm_clock/clock1.jpg)

Although this clock doesn't look pretty, it is likely the most time and effort I have spent on a project as of this writing (31MAR2020). It represents hundreds of hours of work and something that I built from scratch with very few pre-built libraries, no freely given schematics, and only a handful of pre-fabricated PCBs.

This project educated me (the hard way) about knowing and planning the full scope of a project before writing one line of code or placing one part on a schematic. Not knowing the full scope of the project at the beginning resulted in more time spent trying to "future proof" hardware and many firmware "bandages" finding their way into the final code. Had this class lasted a week longer, I likely would have spent that time refactoring much of the code into separate files and functions based on hardware type, and generally cleaning things up. But a ten week term is a ten week term. And sometimes you just need to call something done.

The instructor of this course was insistent that we not use any CAD for the schematics and that much of the work be done in a way that I can best describe as "rugged." So, if you want to see some hand drawn schematics and the final code for this project. Checkout my final submission [git repo](https://github.com/baxterjo/ece473-final)! For something a little "prettier" check out my bluetooth light switch project.
