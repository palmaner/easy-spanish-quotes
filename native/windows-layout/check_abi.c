/* Compile-only comparison with a separately acquired Microsoft SDK kbd.h.
 * The reference header is not redistributed by this project. */
#include <windows.h>
#include <kbd.h>
#include <stddef.h>
#include "layout_abi.h"

#define SIZE(local, reference) _Static_assert(sizeof(local) == sizeof(reference), #local " size")
#define FIELD(local, member, reference, native_member) \
    _Static_assert(offsetof(local, member) == offsetof(reference, native_member), #local "." #member " offset"); \
    _Static_assert(sizeof(((local *)0)->member) == sizeof(((reference *)0)->native_member), #local "." #member " size")

SIZE(ESQ_VK_BIT, VK_TO_BIT);
FIELD(ESQ_VK_BIT, vk, VK_TO_BIT, Vk);
FIELD(ESQ_VK_BIT, bits, VK_TO_BIT, ModBits);
FIELD(ESQ_MODIFIERS, keys, MODIFIERS, pVkToBit);
FIELD(ESQ_MODIFIERS, max, MODIFIERS, wMaxModBits);
_Static_assert(offsetof(ESQ_MODIFIERS, number) == offsetof(MODIFIERS, ModNumber), "modifier tail offset");
_Static_assert(sizeof(((ESQ_MODIFIERS *)0)->number) == 8, "modifier tail capacity");
SIZE(ESQ_CHARS, VK_TO_WCHARS6);
FIELD(ESQ_CHARS, vk, VK_TO_WCHARS6, VirtualKey);
FIELD(ESQ_CHARS, attributes, VK_TO_WCHARS6, Attributes);
FIELD(ESQ_CHARS, chars, VK_TO_WCHARS6, wch);
SIZE(ESQ_CHAR_TABLE, VK_TO_WCHAR_TABLE);
FIELD(ESQ_CHAR_TABLE, rows, VK_TO_WCHAR_TABLE, pVkToWchars);
FIELD(ESQ_CHAR_TABLE, modifications, VK_TO_WCHAR_TABLE, nModifications);
FIELD(ESQ_CHAR_TABLE, stride, VK_TO_WCHAR_TABLE, cbSize);
SIZE(ESQ_DEAD, DEADKEY);
FIELD(ESQ_DEAD, pair, DEADKEY, dwBoth);
FIELD(ESQ_DEAD, composed, DEADKEY, wchComposed);
FIELD(ESQ_DEAD, flags, DEADKEY, uFlags);
SIZE(ESQ_SCAN, VSC_VK);
FIELD(ESQ_SCAN, scan, VSC_VK, Vsc);
FIELD(ESQ_SCAN, vk, VSC_VK, Vk);
SIZE(ESQ_NAME, VSC_LPWSTR);
FIELD(ESQ_NAME, scan, VSC_LPWSTR, vsc);
FIELD(ESQ_NAME, name, VSC_LPWSTR, pwsz);
SIZE(ESQ_TABLES, KBDTABLES);
FIELD(ESQ_TABLES, modifiers, KBDTABLES, pCharModifiers);
FIELD(ESQ_TABLES, characters, KBDTABLES, pVkToWcharTable);
FIELD(ESQ_TABLES, dead, KBDTABLES, pDeadKey);
FIELD(ESQ_TABLES, names, KBDTABLES, pKeyNames);
FIELD(ESQ_TABLES, extended_names, KBDTABLES, pKeyNamesExt);
FIELD(ESQ_TABLES, dead_names, KBDTABLES, pKeyNamesDead);
FIELD(ESQ_TABLES, scans, KBDTABLES, pusVSCtoVK);
FIELD(ESQ_TABLES, scan_count, KBDTABLES, bMaxVSCtoVK);
FIELD(ESQ_TABLES, e0, KBDTABLES, pVSCtoVK_E0);
FIELD(ESQ_TABLES, e1, KBDTABLES, pVSCtoVK_E1);
FIELD(ESQ_TABLES, locale_flags, KBDTABLES, fLocaleFlags);
FIELD(ESQ_TABLES, ligature_max, KBDTABLES, nLgMax);
FIELD(ESQ_TABLES, ligature_stride, KBDTABLES, cbLgEntry);
FIELD(ESQ_TABLES, ligatures, KBDTABLES, pLigature);
FIELD(ESQ_TABLES, type, KBDTABLES, dwType);
FIELD(ESQ_TABLES, subtype, KBDTABLES, dwSubType);
_Static_assert(KBDEXT == 0x100 && KBDMULTIVK == 0x200, "extended/multi-VK flags");
_Static_assert(KBDSPECIAL == 0x400 && KBDNUMPAD == 0x800, "numpad flags");
_Static_assert(WCH_NONE == 0xf000 && WCH_DEAD == 0xf001, "character sentinels");
_Static_assert(CAPLOK == 1 && KLLF_ALTGR == 1 && KBD_VERSION == 1, "locale flags");
