#!/bin/bash

# Setup Cloud-Init User Data
cloud-init devel make-mime \
    -a authorized_keys.yml:cloud-config \
    -a cloud-init/common.yml:jinja2 \
    -a cloud-init/install-dotnet-sdk.yml:jinja2 \
    -a cloud-init/install-golang.yml:jinja2  \
    -a cloud-init/install-rustlang.yml:jinja2 \
    -a cloud-init/install-git-server.yml:jinja2 \
    -a cloud-init/install-$CLUSTER_DRIVER.yml:jinja2 \
    -a cloud-init/install-$SOURCE_DRIVER.yml:jinja2 \
    -a cloud-init/exit.yml:jinja2 > user-data

# Deploy VM
IP_ADDRESS=$(az vm create \
  --resource-group $RESOURCE_GROUP \
  --name $VM_NAME \
  --image $IMAGE \
  --location $LOCATION \
  --admin-username $ADMIN_USERNAME \
  --public-ip-sku Standard \
  --public-ip-address-dns-name "$VM_NAME"-$DEV_ID \
  --generate-ssh-keys \
  --nic-delete-option delete \
  --os-disk-delete-option delete \
  --custom-data user-data \
  --tags dev_id=$DEV_ID | jq -r .publicIpAddress)

# Wait for the machine to be ready
# Note: Things are happening in parallel, so wait a second before trying to connect
sleep 5s
echo "Waiting for the VM to finish Setup ($IP_ADDRESS)"
AWK_TAIL_LOG='/block\t/ {print};/Fetched.+MB/{print "perf \t"$0};/MB.s.+saved/ {print "perf \t"$0};/Cloud-init.+finished.+Up/ {print;exit}'
ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no $IP_ADDRESS "tail -f /var/log/cloud-init-output.log | awk '$AWK_TAIL_LOG'"

# The machine is ready to connect to
# Note: closing an ssh session does weird things with the buffer, give it a second to close
sleep 5s
ssh $IP_ADDRESS