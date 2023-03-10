use std::{env, path::Path};

pub fn setup_ssh(user: &str, server: &str, remote_path: &str, local_path: &str) {
    /* base_ssh = f'{os.environ["HOME"]}/.ssh'
    os.makedirs(base_ssh, exist_ok=True)
    os.mknod(f'{base_ssh}/authorized_keys')

    injected_ssh_config_dir = '/opt/atlassian/pipelines/agent/ssh'
    # The default ssh key with open permissions readable by alt uids
    identity_file = f'{injected_ssh_config_dir}/id_rsa_tmp'
    final_identity_file = f'{base_ssh}/pipelines_id'

    ssh_key = self.get_variable('SSH_KEY')
    if ssh_key:
        self.log_info('Using passed SSH_KEY...')
        with open(final_identity_file, 'w') as f:
            f.write(base64.b64decode(ssh_key).decode())
    elif not os.path.exists(identity_file):
        self.fail('No default SSH key configured in Pipelines.')
    else:
        self.log_info("Configuring ssh with default ssh key.")
        shutil.copyfile(identity_file, final_identity_file)

    # The default known_hosts file
    known_hosts_file = f'{injected_ssh_config_dir}/known_hosts'
    if not os.path.exists(known_hosts_file):
        self.fail(f'No SSH known_hosts configured in Pipelines in {known_hosts_file}.')

    self.log_info('Adding known hosts...')
    with open(known_hosts_file) as f:
        known_hosts = f.read()
    with open(f'{base_ssh}/known_hosts', 'w') as f:
        f.write(known_hosts)

    with open(f'{base_ssh}/config', 'a') as f:
        self.log_info('Appending to ssh config file private key path')
        f.write(f'IdentityFile {final_identity_file}')

    os.chmod(final_identity_file, 0o600)
    os.chmod(base_ssh, mode=stat.S_IRWXU)
    self.log_info('Applied file permissions to ssh directory.') */
}

pub fn deploy(target: &str) {
    println!("Deploying...");
    println!("{}", target);

    let cwd = env::current_dir().unwrap();

    if (target == ".") {
        println!("{}", cwd.display());
        // let path = Path::join(env::current_dir().unwrap().as_path());
    }
}
