#!/bin/bash
clear
echo -n "$ "
sleep 2
echo "whoami" | pv -qL 10
echo "flynn"
echo -n "$ "
sleep 2
echo "uname -a" | pv -qL 10
echo "SolarOS 4.0.1 Generic_50203-02 sun4m i386 Unknown.Unknown"
echo -n "$ "
sleep 2
echo "login -n root" | pv -qL 10
echo "Login incorrect"
echo -n "login: "
sleep 2
echo "backdoor" | pv -qL 10
echo "No home directory specified in password file!"
echo "Logging in with home=/"
echo -n "# "
sleep 2
echo "bin/history" | pv -qL 10
echo "  488 cd /opt/LLL/controller/laser/"
echo "  489 vi LLLSDLaserControl.c"
echo "  490 make"
echo "  491 make install"
echo "  492 ./sanity_check"
echo "  493 ./configure -o test.cfg"
echo "  494 vi test.cfg"
echo "  495 vi ~/last_will_and_testament.txt"
echo "  496 cat /proc/meminfo"
echo "  497 ps -a -x -u"
echo "  498 kill -9 2207"
echo "  499 kill 2208"
echo "  500 ps -a -x -u"
echo "  501 touch /opt/LLL/run/ok"
echo "  502 LLLSDLaserControl -ok 1"
echo -n "# "
sleep 2
echo "bin/LLLSDLaserControl -ok 1" | pv -qL 10
echo "  * Starting up..."
echo "  * PSU online"
echo "  * HV online"
echo -n "  * Analog core memory... "
sleep 0.5
echo "OK!"
echo "  * Booting pattern recognition systems"
echo "  * Merging current data model"
echo "  * Starting laser emitter"
sleep 0.5
echo "  * Particle traps test OK!"
echo "  * Entangling laser with particle traps"
for i in $(seq 10)
do
  echo -ne "\x1b[10A\x1b[26C\x1b[30;43m                           \x1b[0m\n\x1b[26C\x1b[30;43m      Aperture Clear?      \x1b[0m\n\x1b[26C\x1b[30;43m                           \x1b[0m\n\x1b[26C\x1b[30;43m  < \x1b[37;40mYes\x1b[30;43m >          < No >  \x1b[0m\n\x1b[26C\x1b[30;43m                           \x1b[0m\n\x1b[9B"
  sleep 0.5
  echo -ne "\x1b[10A\x1b[26C\x1b[30;43m                           \x1b[0m\n\x1b[26C\x1b[30;43m      Aperture Clear?      \x1b[0m\n\x1b[26C\x1b[30;43m                           \x1b[0m\n\x1b[26C\x1b[30;43m  < \x1b[30;43mYes\x1b[30;43m >          < No >  \x1b[0m\n\x1b[26C\x1b[30;43m                           \x1b[0m\n\x1b[9B"
  sleep 0.5
done
echo -ne "\x1b[H\x1b[37;47m"
for y in $(seq $LINES)
do
  for x in $(seq $COLUMNS)
  do
    echo -n " "
  done
  echo ""
done