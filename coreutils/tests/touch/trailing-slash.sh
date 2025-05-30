#!/bin/sh
# Ensure that touch honors trailing slash.

# Copyright (C) 2009-2024 Free Software Foundation, Inc.

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

ln -s nowhere dangling || framework_failure_
ln -s loop loop || framework_failure_
touch file || framework_failure_
ln -s file link1 || framework_failure_
mkdir dir || framework_failure_
ln -s dir link2 || framework_failure_


# Trailing slash can only appear on directory or symlink-to-directory.
# Up through coreutils 8.0, Solaris 9 failed these tests.
returns_ 1 touch no-file/ || fail=1
returns_ 1 touch file/ || fail=1
returns_ 1 touch dangling/ || fail=1
returns_ 1 touch loop/ || fail=1
if returns_ 2 ls link1/; then  # darwin allows trailing slash to files
  returns_ 1 touch link1/ || fail=1
fi
touch dir/ || fail=1

# -c silences ENOENT, but not ENOTDIR or ELOOP
touch -c no-file/ || fail=1
returns_ 1 touch -c file/ || fail=1
touch -c dangling/ || fail=1
returns_ 1 touch -c loop/ || fail=1
if returns_ 2 ls link1/; then
  returns_ 1 touch -c link1/ || fail=1
fi
touch -c dir/ || fail=1
returns_ 1 test -f no-file || fail=1
returns_ 1 test -f nowhere || fail=1

# Trailing slash dereferences a symlink, even with -h.
# mtime is sufficient to show pass (besides, lstat changes atime of
# symlinks and directories under Cygwin 1.5).
touch -d 2009-10-10 -h link2/ || fail=1
touch -h -r link2/ file || fail=1
case $(stat --format=%y dir) in
  2009-10-10*) ;;
  *) fail=1 ;;
esac
case $(stat --format=%y link2) in
  2009-10-10*) fail=1 ;;
esac
case $(stat --format=%y file) in
  2009-10-10*) ;;
  *) fail=1 ;;
esac

Exit $fail
