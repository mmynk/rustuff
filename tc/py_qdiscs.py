from pprint import pprint

from pyroute2 import IPRoute
from pyroute2.netlink.rtnl import RTM_GETQDISC

def main():
    ip = IPRoute()
    qdiscs = ip.get_qdiscs(index=3)
    pprint(qdiscs)

    classes = ip.get_classes(index=3)
    pprint(classes)

if __name__ == '__main__':
    main()
