#include <pci/pci.h>
#include <pci/types.h>
#include <stdio.h>
#include <stdlib.h>

int i = 0;

char *parse_vendor(u16 vendor_id)
{
    // printf("%d\n", vendor_id);
    /*switch (vendor_id)
    {
    case 0x0300:
        return "AMD";
        break;
    case 0x1002:
        return "ATI";
        break;
    case 0x10de:
        return "NVIDIA";
        break;
    default:
        return "";
        break;
    }*/
    switch (vendor_id)
    {

//%REPLACE%
    default:
        return "UnknOwOwn VendOwOr :(";
    }
}

char **get_gpu()
{
    struct pci_access *pciaccess;
    struct pci_dev *dev;
    char **names;

    pciaccess = pci_alloc();
    pci_init(pciaccess);
    pci_scan_bus(pciaccess);
    i = 0;
    names = (char **)malloc(512 * (2048 * sizeof(char *)));

    for (dev = pciaccess->devices; dev; dev = dev->next)
    {
        pci_fill_info(dev, PCI_FILL_IDENT | PCI_FILL_CLASS);
        switch (dev->device_class) {
            case 0x3080:
            case 0x0301:
            case 0x0302:
            case 0x0300:
                char *name = (char *)malloc(2048 * sizeof(char));
                char name_buffer[1024];
                pci_lookup_name(pciaccess, name_buffer, sizeof(name_buffer),
                                PCI_LOOKUP_DEVICE, dev->vendor_id, dev->device_id);
                sprintf(name, "%s %s", parse_vendor(dev->vendor_id),
                        name_buffer);
                if (i >= 512)
                    break;
                names[i] = name;
                i++;
            default:
                continue;
        }
    }
    pci_cleanup(pciaccess);
    return names;
}

int get_gpu_count() {
    return i;
}

/*int main()
{
    struct pci_access* pciaccess;
        struct pci_dev* dev;
        //char name_buffer[1024];

        pciaccess = pci_alloc();
        pci_init(pciaccess);
        pci_scan_bus(pciaccess);

        for(dev = pciaccess->devices; dev; dev = dev->next)
        {
                pci_fill_info(dev, PCI_FILL_IDENT | PCI_FILL_BASES |
PCI_FILL_CLASS);
                //printf(" (%s)\n", name_buffer);
                if (dev->device_class == 0x0300) {
                    printf("owo\n");
                    char name_buffer[1024];
                    pci_lookup_name(pciaccess, name_buffer, sizeof(name_buffer),
PCI_LOOKUP_DEVICE, dev->vendor_id, dev->device_id); printf("%s %s\n%04x\n",
parse_vendor(dev->vendor_id), name_buffer, dev->vendor_id); break;
                }
                printf("%d\n", dev->device_class);
        }

        pci_cleanup(pciaccess);
        return 0;
    printf("GPU: %s\n", get_gpu());
}
*/
