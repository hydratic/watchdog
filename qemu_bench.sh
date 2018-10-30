
# a shell script that uses qemu to benchmark the amount of ram used when
# running watchdog on the x86 emulator.

install_x86()
{
  qemu -hda watchdog.img -boot d -cdrom ./watchdog_test.iso -m 4096
}

test_x86()
{
  cd watchdog
  make
  qemu-img create watchdog.img 5G
  
  qemu_ifup()
  ifconfig br0
  ifconfig tap0
  
  qemu -m 4096 -hda watchdog.img -net nic -net tap,ifname=tap0,script=no
}

qemu_ifup()
{
  /sbin/ifconfig eth1 down
  /sbin/ifconfig eth1 0.0.0.0 promisc up
  openvpn --mktun --dev tap0
  ifconfig tap 0 0.0.0.0 up
  brctl addbr br0
  brctl addif br0 eth1
  brctl addif br0 tap0
  brctl stp br0 off
  ifconfig br0 10.10.10.2 netmask
  255.255.255.0
}

qemu_ifdown()
{
  ifconfig eth1 down
  ifconfig eth1 -promisc
  ifup eth1
  ifconfig br0 down
  brctl delbr br0
  openvpn --rmtun --dev tap0
}
