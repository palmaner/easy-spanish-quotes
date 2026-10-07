# Milestone ledger

| Milestone | Actual state |
|---|---|
| Repository initialization | Public repo, approved specification/license/README committed and pushed |
| M0 | Research/specification, read-only fixtures, native emitter/build and inspector implemented; automated checks pass |
| M1 | Experimental native lifecycle and selection code implemented; real integration/WDK gates pending, NOT accepted |
| M2 | Receipt ownership and intent journaling, core recovery protocol/tests implemented; complete native rollback, separate production elevation and upgrades pending |
| M3 | Spanish Win32 GUI preview implemented; signed installer, accessibility matrix and supported release pending |
| M4 | One-preset unsigned measurement recorded; signing/650-variant costs unmeasured; expansion deferred |
| M5–M7 | Detailed approved specification preserved; native backends/architectures not yet implemented |

Hardware tests remain pending at the user's request. No supported release is
published and no layout is installed on the developer's working machine.

Next native work: compare the independently declared descriptor/scancode flags
against current WDK; run W1–W5 in a disposable VM; measure actual user-state
deltas; implement precise default-policy restoration; close all partial-native
recovery cases. Production architecture commitments remain behind this gate.
