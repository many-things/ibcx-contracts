#!/bin/bash

HOME=$PWD/localnet

cd osmosis
HOME=$HOME make localnet-stop
cd ..
