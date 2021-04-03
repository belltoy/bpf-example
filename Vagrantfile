["vagrant-reload", "vagrant-vbguest"].each do |plugin|
  unless Vagrant.has_plugin?(plugin)
    raise "Vagrant plugin #{plugin} is not installed!"
  end
end

Vagrant.configure('2') do |config|
  config.vagrant.plugins = ["vagrant-reload", "vagrant-vbguest"]

  # set auto_update to false, if you do NOT want to check the correct
  # additions version when booting this machine
  config.vbguest.auto_update = false
  config.vbguest.installer_options = { running_kernel_modules: ["vboxguest", "vboxsf"] }

  config.vm.box = "ubuntu/focal64"
  config.vm.network "private_network", type: "dhcp"
  config.vm.box_check_update = false

  config.vm.synced_folder ".", "/vagrant", type: "virtualbox", disabled: true
  config.vm.synced_folder ".", "/workspace/bpf-example", type: "virtualbox", automount: true

  config.vm.provider "virtualbox" do |vb|
    # Customize the amount of memory on the VM:
    vb.memory = "4096"
  end

  config.vm.provision "shell", :privileged => true, inline: <<-SHELL
    export DEBIAN_FRONTEND=noninteractive
    export APT_KEY_DONT_WARN_ON_DANGEROUS_USAGE=1
    cp /workspace/bpf-example/etc/sources.163.list /etc/apt/sources.list
    cp /workspace/bpf-example/etc/sources.llvm-11.list /etc/apt/sources.list.d/llvm-11.list
    wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
    apt-get update
    apt-get install -y build-essential zlib1g-dev llvm-11-dev libclang-11-dev linux-headers-$(uname -r)
  SHELL

  config.vm.provision "shell", :privileged => false, inline: <<-SHELL
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  SHELL

  config.vm.provision "shell", :privileged => false, inline: <<-SHELL
    cargo install cargo-bpf
    echo 'cd /workspace/bpf-example' >> ~/.bashrc
  SHELL
end
