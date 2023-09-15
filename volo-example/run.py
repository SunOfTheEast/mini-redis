import argparse

def parse_config(config_path):
    with open(config_path, 'r') as f:
        lines = f.readlines()

    is_proxy = False
    is_master = False
    is_slave = False

    proxy_info = {}
    master_slaves = {}

    for line in lines:
        line = line.strip()
        if '[proxy]' in line:
            is_proxy = True
            is_master = False
            is_slave = False
        elif '[master server]' in line:
            is_master = True
            is_proxy = False
            is_slave = False
        elif '[slaver server]' in line:
            is_slave = True
            is_master = False
            is_proxy = False
        elif is_proxy and 'port:' in line:
            current_proxy = line.split(':')[-1].strip()
            proxy_info[current_proxy] = []
        elif is_proxy and 'colony:' in line:
            master_ports = line.split(':')[-1].strip().split()
            proxy_info[current_proxy] = master_ports
        elif is_master and 'port:' in line:
            current_master = line.split(':')[-1].strip()
        elif is_slave and 'port:' in line:
            slave_ports = line.split(':')[-1].strip().split()
            master_slaves[current_master] = slave_ports

    return proxy_info, master_slaves

def generate_bash_script(proxy_info, master_slaves, specific_slaves=None, all_slaves=False):
    bash_script = "#!/bin/bash\n"
    bash_script += "cd /mnt/c/Users/23215/Desktop/mini-redis-master/volo-example\n"

    for proxy, masters in proxy_info.items():
        bash_script += f"cargo run --bin server {proxy} -p {' '.join(masters)} &\n"
        for master in masters:
            bash_script += f"cargo run --bin server {master} -m {' '.join(master_slaves[master])} &\n"

    if all_slaves:
        for master, slaves in master_slaves.items():
            for slave in slaves:
                bash_script += f"cargo run --bin server {slave} -s {master} &\n"
    elif specific_slaves:
        for slave_port, master_port in specific_slaves:
            bash_script += f"cargo run --bin server {slave_port} -s {master_port} &\n"

    bash_script += "wait\n"

    with open("run_servers.sh", 'w') as f:
        f.write(bash_script)

def main():
    parser = argparse.ArgumentParser(description="Generate bash script from config")
    parser.add_argument('-s', '--slaves', nargs='*', help="Specific slaves to run", default=argparse.SUPPRESS)
    args = parser.parse_args()

    proxy_info, master_slaves = parse_config('config.txt')

    specific_slave_tuples = None
    all_slaves_flag = False
    if 'slaves' in args:
        if args.slaves:
            # Construct list of tuples containing (slave_port, master_port) for specific slaves
            specific_slave_tuples = [(slave, next(master for master, slaves in master_slaves.items() if slave in slaves)) for slave in args.slaves]
        else:
            all_slaves_flag = True

    generate_bash_script(proxy_info, master_slaves, specific_slave_tuples, all_slaves_flag)

if __name__ == "__main__":
    main()
