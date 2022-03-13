#include <pci/pci.h>
#include <pci/types.h>
#include <stdio.h>
#include <stdlib.h>

int i = 0;

char **get_gpu()
{
    struct pci_access *pciaccess;
    struct pci_dev *dev;
    char **names;

    pciaccess = pci_alloc();
    pci_init(pciaccess);
    pci_scan_bus(pciaccess);
    i = 0;
    names = (char **)malloc(512 * sizeof(char *));

    for (dev = pciaccess->devices; dev; dev = dev->next)
    {
        char *full_name = (char *)malloc(2048 * sizeof(char));
        char *vendor_buf = (char *)malloc(1024 * sizeof(char));
        char name_buf[1024] = {'\0'};

        pci_fill_info(dev, PCI_FILL_IDENT | PCI_FILL_CLASS | PCI_FILL_LABEL);
        switch (dev->device_class)
        {
        case 0x0380:
        case 0x0301:
        case 0x0302:
        case 0x0300:
            pci_lookup_name(pciaccess, name_buf, 1024 * sizeof(char),
                            PCI_LOOKUP_DEVICE, dev->vendor_id, dev->device_id);
            pci_lookup_name(pciaccess, vendor_buf, 1024 * sizeof(char),
                            PCI_LOOKUP_VENDOR, dev->vendor_id, dev->device_id);

            sprintf(full_name, "%s %s", vendor_buf,
                    name_buf);
            if (i > 512) {
                pci_cleanup(pciaccess);
                free(vendor_buf);
                return names;
            }
            names[i] = full_name;
            i++;
        default:
            break;
        }
        free(vendor_buf);
    }
    pci_cleanup(pciaccess);
    return names;
}

int get_gpu_count()
{
    return i;
}