#!/bin/bash

echo "Creating VM with:"
echo "resource_group: $RESOURCE_GROUP"
echo "vm_name:        $VM_NAME"
echo "image:          $IMAGE"
echo "location:       $LOCATION"
echo "admin_user:     $ADMIN_USERNAME"
echo "dns:            $VM_NAME-$DEV_ID.$LOCATION.cloudapp.azure.com"

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
  --custom-data user_data \
  --tags dev_id=$DEV_ID | jq -r .publicIpAddress)

# Wait for the machine to be ready
# Note: Things are happening in parallel, so wait a second before trying to connect
# sleep 5s
# echo "Waiting for the VM to finish Setup ($IP_ADDRESS)"
# AWK_TAIL_LOG='/block\t/ {print};/Fetched.+MB/{print "perf \t"$0};/MB.s.+saved/ {print "perf \t"$0};/Cloud-init.+finished.+Up/ {print;exit}'
# ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no $IP_ADDRESS "tail -f /var/log/cloud-init-output.log | awk '$AWK_TAIL_LOG'"

echo "VM is ready at $IP_ADDRESS"
