#!/usr/bin/env bash

likwid-perfctr -C 0 -g MEM -m -- cargo test
