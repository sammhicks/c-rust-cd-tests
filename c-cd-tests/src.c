#include <stdlib.h>
#include <stdio.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <string.h>
#include <errno.h>
#include <sys/ioctl.h>
#include <linux/cdrom.h>

int main() {
    int r;

    int fd = open("/dev/cdrom", O_RDONLY);

    if (fd < 0) {
        printf("Failed to open file: %s\n", strerror(errno));
        return 1;
    }

    struct cdrom_tochdr th;

    r = ioctl(fd, CDROMREADTOCHDR, &th);

    if (r < 0) {
        printf("Failed to get TOC: %s\n", strerror(errno));
        return 1;
    }

    printf("CdToc { cdth_trk0: %d, cdth_trk1: %d }\n", th.cdth_trk0, th.cdth_trk1);

    for (int track_num = th.cdth_trk0; track_num <= th.cdth_trk1; ++track_num) {
        struct cdrom_tocentry te;
        te.cdte_track = track_num;
        te.cdte_format = CDROM_LBA;

        r = ioctl(fd, CDROMREADTOCENTRY, &te);

        if (r < 0) {
            printf("Failed to get TOC Entry: %s\n", strerror(errno));
            return 1;
        }

        printf("CdTocEntry { cdte_track: %u, adr: %u, ctrl: %u, cdte_format: %u, cdte_lba: %u, cdte_datamode: %x }\n",
            te.cdte_track,
            te.cdte_adr,
            te.cdte_ctrl,
            te.cdte_format,
            te.cdte_addr.lba,
            te.cdte_datamode
        );
    }

    struct cdrom_mcn mcn;
    memset(&mcn, 0, sizeof mcn);

    r = ioctl(fd, CDROM_GET_MCN, &mcn);

    if (r < 0) {
        printf("Failed to get MCN Entry: %s\n", strerror(errno));
        return 1;
    }

    printf("mcn: %s\n", (unsigned char *)&mcn.medium_catalog_number);
}
