#!/bin/bash
echo "Report of $USER"
echo " +-------------------+ "
echo "Network config:"
ifconfig
echo "OS Version and info:"
uname -a
echo "Public IP:"
curl icanhazip.com
echo " +-------------------+ "