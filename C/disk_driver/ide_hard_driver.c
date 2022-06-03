// Keep polling untill noy busy
static int ide_wait_ready() {
    while(((int r = inb(0x1f7)) & IDE_BSY) || !(r & IDE_DRDY))
        ;
}

static void ide_start_request(struct buf* b) {
    ide_wait_ready();
    outb(0x3f6, 0);
    outb(0x1f2, 1);
    outb(0x1f3, b->sector & 0xff);
    outb(0x1f4, (b->sector >> 8) & 0xff);
    outb(0x1f5, (b->sector >> 16) & 0xff);
    outb(0x1f6, 0xe0 | (b->dev & 1)<<4 | ((b->sector>>24) & 0x0f));
    if(b->flags & B_DIRTY) {
        outb(0x1f7, IDE_CMD_WRITE);
        outsl(0x1f0, b->data, 512/4);
    } else {
        outb(0x1f7, IDE_CMD_READ);
    }
}

void ide_rw(struct buf* b) {
    acquire(&ide_lock);
    for(struct buf** pp = &ide_queue; *pp; pp = &(*pp)->qnext)
        ;
    *pp = b;
    if(ide_queue == b)
        ide_start_request(b);
    while((b->flags & (B_VALID | B_DIRTY)) != B_VALID)
        sleep(b, &ide_lock);
    release(&ide_lock);
}

void ide_intr() {
    struct buf* b;
    acquire(&ide_lock);
    if(!(b->flags & B_DIRTY) && ide_wait_ready() >= 0)
        insl(0x1f0, b->data, 512/4);
    b->flags |= B_VALID;
    b->flags &= ~B_DIRTY;
    wakeup(b);
    if((ide_queue = b->qnext) != 0)
        ide_start_request(ide_queue);
    release(&ide_lock);
}


