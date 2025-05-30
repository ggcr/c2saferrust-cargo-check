#!/bin/sh
# Demonstrate that "touch -d now writable-but-owned-by-other" works.

# Copyright (C) 2008-2024 Free Software Foundation, Inc.

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
. "$SCRIPTPATH/../../tests/init.sh"; path_prepend_ $1
require_root_

# Create a file owned by root, and writable by $NON_ROOT_USERNAME.
echo > root-owned || framework_failure_
chgrp +$NON_ROOT_GID . root-owned || framework_failure_
chmod g+w root-owned

# Ensure that the current directory is searchable by $NON_ROOT_USERNAME.
chmod g+x .

chroot --skip-chdir --user=$NON_ROOT_USERNAME / env PATH="$PATH" \
  touch -d now root-owned || fail=1

Exit $fail
