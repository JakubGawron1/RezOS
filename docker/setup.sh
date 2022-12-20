#!/bin/bash

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.


# The docker image generated by this will be used as a build environment.
# The following command generated the image based on the Dockerfile in the working directory,
# for the x86_64 architecture and names it 'rezenv'
docker build . --platform amd64 -t rezenv
