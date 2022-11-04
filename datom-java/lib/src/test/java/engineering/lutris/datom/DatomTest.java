// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

package engineering.lutris.datom;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class DatomTest {
    @Test
    void versionWorks() {
        assertEquals(Datom.VERSION, "0.1.1-pre4");
    }
}
