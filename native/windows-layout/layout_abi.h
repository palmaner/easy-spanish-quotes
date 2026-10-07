/* Independently declared public Windows keyboard-layout ABI. No SDK source copied.
 * Must be checked against the installed WDK before supported release. */
#pragma once
#include <stdint.h>
typedef struct { uint8_t vk, bits; } ESQ_VK_BIT;
typedef struct { ESQ_VK_BIT *keys; uint16_t max; uint8_t number[8]; } ESQ_MODIFIERS;
typedef struct { uint8_t vk, attributes; uint16_t chars[6]; } ESQ_CHARS;
typedef struct { ESQ_CHARS *rows; uint8_t modifications, stride; } ESQ_CHAR_TABLE;
typedef struct { uint32_t pair; uint16_t composed, flags; } ESQ_DEAD;
typedef struct { uint8_t scan; uint16_t vk; } ESQ_SCAN;
typedef struct { uint8_t scan; const uint16_t *name; } ESQ_NAME;
typedef struct {
    ESQ_MODIFIERS *modifiers;
    ESQ_CHAR_TABLE *characters;
    ESQ_DEAD *dead;
    ESQ_NAME *names, *extended_names;
    const uint16_t **dead_names;
    uint16_t *scans;
    uint8_t scan_count;
    ESQ_SCAN *e0, *e1;
    uint32_t locale_flags;
    uint8_t ligature_max, ligature_stride;
    void *ligatures;
    uint32_t type, subtype;
} ESQ_TABLES;
_Static_assert(sizeof(ESQ_CHARS) == 14, "character row ABI");
_Static_assert(sizeof(ESQ_DEAD) == 8, "dead key ABI");
_Static_assert(sizeof(void *) != 8 || sizeof(ESQ_TABLES) == 104, "x64 descriptor ABI");
