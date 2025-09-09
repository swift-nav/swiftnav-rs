# swiftnav

`swiftnav` is a library that implements GNSS utility functions to perform
position estimations. The data used by `swiftnav` typically comes from GNSS
receiver chips as raw observation and ephemeris data. `swiftnav` is more of
a "bring your own algorithm" library, it provides a bunch of functionality that
is useful when processing raw GNSS data, but it provides only limited position
estimation capabilities. Each module encompasses a single set of functionality,
and they are meant to be pretty self-explanatory for developers familiar with
GNSS processing.

GNSS systems are used to estimate the location of the receiver by determining
the distance between the receiver and several satellites. The satellites send
out precisely timed periodic messages and the receiver measures the delay
of those messages. Knowing the location of the satellites at the time of
transmission and the delays of the messages the receiver is able to determine
the location of itself in relation to the satellites.

`swiftnav` does not provide any functionality for communicating with
receivers made by Swift Navigation, or any manufacturer.
[libsbp](https://github.com/swift-nav/libsbp) is the library to use if you
want to communicate with receivers using Swift Binary Protocol (SBP).