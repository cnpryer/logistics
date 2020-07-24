# -*- mode: ruby -*-
# vi: set ft=ruby :


BOX = "bento/ubuntu-20.04"
SCRIPT = "./vm-bootstrap.sh"

Vagrant.configure(2) do |config|

    config.vm.box = BOX

    # Install Docker
    config.vm.provision :docker

    # Install Docker Compose
    # First, install required plugin https://github.com/leighmcculloch/vagrant-docker-compose:
    # vagrant plugin install vagrant-docker-compose
    config.vm.provision :docker_compose

    config.vm.provision "shell", path: SCRIPT
  end