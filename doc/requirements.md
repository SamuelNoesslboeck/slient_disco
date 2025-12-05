# Requirements of a Silent Disco Broadcast Server

**By Maximilian Heidenreich, February 4, 2025**

## Introduction

Silent discos are the perfect way to listen to music on small or big events in urban areas. It allows every
participant to set the preferred volume on their headphones. Current solutions require renting a hole setup,
with a special transmitter and several special receiver headphones. Using existing hardware would be desirable,
as it solves issues with theft, hygiene, and expenses hosting events.

## Requirements

The requirements on the proposed streaming solution are:
• short latency of best TRT T ≤ 10ms
• easy and fast access
• platform independent usability
• requiring no special equipment besides in-house hardware

## The idea

A Windows and Mac Software is needed, which the DJ installs on the Computer playing the music. This
Software shall listen to the Output of the DJ mixing-software, or should be selectable as a speaker in the
system settings. As another option, one could also set up a separate Laptop with the corresponding software,
which can be feed with auxiliary cables from the DJs setup.
The software then should broadcast the audio into a local network and convert the device on which its installed
to a Server, that can be accessed by the crowd. The crowd should have the possibility to access that intranet
and server fast and easy, best via scanning a QR Code that first connects to a Wifi and then opens a website
in the browser. In comparison to other approaches, no additional application should be needed. This website
should only include a play button.
The party guests then can listen to the music through their own phones, that can be connected to their
personal headphones. Here it must be guaranteed that all guests listen quasi-synchronously and that their
music is synchronous enough to a quietly playing speaker setup directly connected to the DJ. Also for the DJ,
the music played out silently as a monitor must be synchronous enough to the crowd. Hence the short latency
from the server to the party guests.
The Computer, on which the software is set up, should be directly connected to a Wifi router. That router
might be limited in connections, therefore it must be possible to split the load onto several access points. This
also has to work within the demanded latency.
In a next step, it should be prohibited for the guests to infiltrate the DJs System or the setup via their
connection