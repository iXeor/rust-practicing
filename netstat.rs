// Here is an rust version netstat command
// It can be used to get the network status of the current machine

// The structure of netstat file for storage of network status
struct Netstat {
    slot: String,
    recv_q: String,
    send_q: String,
    local_addr: String,
    foreign_addr: String,
    state: String,
    pid: String,
}

// The enumeration of the netstat state for storage of network status
enum NetstatState {
    ESTABLISHED,
    SYN_SENT,
    SYN_RECV,
    FIN_WAIT1,
    FIN_WAIT2,
    TIME_WAIT,
    CLOSE,
    CLOSE_WAIT,
    LAST_ACK,
    LISTEN,
    CLOSING,
    UNKNOWN,
}

// The function for get processname by pid from netstat structure
// Input: a vector of Netstat structure and a pid
// Output: a string of processname
// The function will return a string of processname

fn get_processname(netstat: &Vec<Netstat>, pid: &str) -> String {
    // The processname
    let mut processname = String::new();
    // The path of the process status file
    let path = "/proc/" + pid + "/status";
    // The file for reading
    let file = File::open(path).unwrap();
    // The reader for reading file
    let reader = BufReader::new(file);
    // The iterator for reading file line by line
    let mut lines = reader.lines();
    // Read the file line by line
    for line in lines {
        // The line is a string
        let line = line.unwrap();
        // Split the line by space
        let mut line = line.split_whitespace();
        // The first element is the key
        let key = line.next().unwrap().to_string();
        // The second element is the value
        let value = line.next().unwrap().to_string();
        // If the key is "Name:", then the value is the processname
        if key == "Name:" {
            processname = value;
            break;
        }
    }
    // Return the processname
    processname
}


// The function to get the network status from file
// Input: target protocol for reading
// Output: a vector of Netstat structure in readable format
// The protocol is a string, which can be "tcp" or "udp" or "tcp6" or "udp6"
// And the protocol is case insensitive for opening target file.
// The function will return a vector of Netstat structure, which can be used to print the network status

fn get_netstat(protocol: &str) -> Vec<Netstat> {
    // The path of netstat file
    let path = "/proc/net/" + protocol.to_lowercase();
    // The vector for storing the network status
    let mut netstat: Vec<Netstat> = Vec::new();
    // The file for reading
    let file = File::open(path).unwrap();
    // The reader for reading file
    let reader = BufReader::new(file);
    // The iterator for reading file line by line
    let mut lines = reader.lines();
    // The first line is the title of the file, so we need to skip it
    lines.next();
    // Read the file line by line
    for line in lines {
        // The line is a string
        let line = line.unwrap();
        // Split the line by space
        let mut line = line.split_whitespace();
        // The first element is the slot
        let slot = line.next().unwrap().to_string();
        // The second element is the recv_q
        let recv_q = line.next().unwrap().to_string();
        // The third element is the send_q
        let send_q = line.next().unwrap().to_string();
        // The fourth element is the local_addr
        let local_addr = line.next().unwrap().to_string();
        // The fifth element is the foreign_addr
        let foreign_addr = line.next().unwrap().to_string();
        // The sixth element is the state
        let state = line.next().unwrap().to_string();
        // The seventh element is the pid
        let pid = line.next().unwrap().to_string();
        // Create a Netstat structure
        let netstat_line = Netstat {
            slot,
            recv_q,
            send_q,
            local_addr,
            foreign_addr,
            state,
            pid,
        };
        // Push the Netstat structure into vector
        netstat.push(netstat_line);
    }
    // Return the vector
    netstat
}

// The filter function for filtering the network status by pid
// Input: a vector of Netstat structure and a pid in integer type
// Output: a vector of Netstat structure filtered by pid
// The function will return a vector of Netstat structure, which can be used to print the network status

fn filter_by_pid(netstat: Vec<Netstat>, pid: i32) -> Vec<Netstat> {
    // The vector for storing the network status filtered by pid
    let mut netstat_filtered: Vec<Netstat> = Vec::new();
    // Iterate the vector
    for netstat_line in netstat {
        // If the pid of the line is equal to the pid, push it into vector
        if netstat_line.pid == pid.to_string() {
            netstat_filtered.push(netstat_line);
        }
    }
    // Return the vector
    netstat_filtered
}

// The function for converting the NetStatState to string
// Input: a enum value of NetStatState
// Output: a string of state value
// e.g: Input is NetStatState::LISTEN, Output is "0A"

fn state_to_string(state: NetstatState) -> String {
    // Convert the state to string
    let state_string = match state {
        NetstatState::ESTABLISHED => "01",
        NetstatState::SYN_SENT => "02",
        NetstatState::SYN_RECV => "03",
        NetstatState::FIN_WAIT1 => "04",
        NetstatState::FIN_WAIT2 => "05",
        NetstatState::TIME_WAIT => "06",
        NetstatState::CLOSE => "07",
        NetstatState::CLOSE_WAIT => "08",
        NetstatState::LAST_ACK => "09",
        NetstatState::LISTEN => "0A",
        NetstatState::CLOSING => "0B",
        NetstatState::UNKNOWN => "0C",
    };
    // Return the string
    state_string.to_string()
}

// The filter function for filtering the network status by state
// Input: a vector of Netstat structure and a state in NetstatState type
// Output: a vector of Netstat structure filtered by state
// The function will return a vector of Netstat structure, which can be used to print the network status

fn filter_by_state(netstat: Vec<Netstat>, state: NetstatState) -> Vec<Netstat> {
    // The vector for storing the network status filtered by state
    let mut netstat_filtered: Vec<Netstat> = Vec::new();
    // Iterate the vector
    for netstat_line in netstat {
        // If the state of the line is equal to the state, push it into vector
        if netstat_line.state == state_to_string(state) {
            netstat_filtered.push(netstat_line);
        }
    }
    // Return the vector
    netstat_filtered
}

// The function for get ip from local_addr or foreign_addr
// Input: a string of local_addr or foreign_addr
// Output: a string of ip
// The function will return a string of ip

fn get_ip(addr: &str) -> String {
    // Split the addr by :
    let mut addr = addr.split(":");
    // The first element is the ip
    let ip = addr.next().unwrap().to_string();
    // Return the ip
    ip
}

// The function for get port from local_addr or foreign_addr
// Input: a string of local_addr or foreign_addr
// Output: a string of port
// The function will return a string of port

fn get_port(addr: &str) -> String {
    // Split the addr by :
    let mut addr = addr.split(":");
    // The first element is the ip
    let ip = addr.next().unwrap().to_string();
    // The second element is the port
    let port = addr.next().unwrap().to_string();
    // Return the port
    port
}

// The filter function for filtering the network status by port
// Input: a vector of Netstat structure and a port
// Output: a vector of Netstat structure filtered by port
// The function will return a vector of Netstat structure, which can be used to print the network status

fn filter_by_port(netstat: Vec<Netstat>, port: &str) -> Vec<Netstat> {
    // The vector for storing the network status filtered by port
    let mut netstat_filtered: Vec<Netstat> = Vec::new();
    // Iterate the vector
    for netstat_line in netstat {
        // Get the port of local_addr
        let local_port = get_port(&netstat_line.local_addr);
        // Get the port of foreign_addr
        let foreign_port = get_port(&netstat_line.foreign_addr);
        // If the port of local_addr or foreign_addr is equal to the port, push it into vector
        if local_port == port || foreign_port == port {
            netstat_filtered.push(netstat_line);
        }
    }
    // Return the vector
    netstat_filtered
}

// The function for check the system is in big endian or little endian
// Input: None
// Output: a bool value, true for big endian, false for little endian
// The function will return a bool value, true for big endian, false for little endian

fn is_big_endian() -> bool {
    // Create a u16 value
    let x: u16 = 0x1234;
    // Convert the u16 value to a u8 array
    let x: [u8; 2] = unsafe { mem::transmute(x) };
    // If the first element of the array is 0x12, the system is big endian
    if x[0] == 0x12 {
        true
    } else {
        false
    }
}

// The function for convert IP and port to readable format
// Input: a string of IP and port
// Output: a string of IP and port in readable format
// The function will return a string of IP and port in readable format in proper endian mode

fn ip_port_readable(ip_port: &str) -> String {
    // The IP and port is a string
    let ip_port = ip_port.to_string();
    // Split the IP and port by colon
    let mut ip_port = ip_port.split(":");
    // The first element is the IP
    let ip = ip_port.next().unwrap().to_string();
    // The second element is the port
    let port = ip_port.next().unwrap().to_string();
    // Convert the IP to u32
    let ip = u32::from_str_radix(&ip, 16).unwrap();
    // Convert the port to u32
    let port = u32::from_str_radix(&port, 16).unwrap();
    // If the system is big endian, convert the IP and port to big endian
    if is_big_endian() {
        // Convert the IP to big endian
        let ip = ip.to_be();
        // Convert the port to big endian
        let port = port.to_be();
        // Convert the IP to string
        let ip = format!("{:08X}", ip);
        // Convert the port to string
        let port = format!("{:04X}", port);
        // Return the IP and port in readable format
        ip + ":" + &port
    } else {
        // Convert the IP to little endian
        let ip = ip.to_le();
        // Convert the port to little endian
        let port = port.to_le();
        // Convert the IP to string
        let ip = format!("{:08X}", ip);
        // Convert the port to string
        let port = format!("{:04X}", port);
        // Return the IP and port in readable format
        ip + ":" + &port
    }
}

// The function for filtering the network status by port and returns a list of Process Name
// Input: a bool for select local addr or foreign addr, a vector of Netstat structure and a port in integer
// Output: a string vector of Process Name filtered by port
// The function will return a vector of Process Name, which can be used to print the network status

fn filter_by_port_processname(
    is_local_addr: bool,
    netstat: Vec<Netstat>,
    port: u16,
) -> Vec<String> {
    // The vector for storing the processname filtered by port
    let mut processname_filtered: Vec<String> = Vec::new();
    // Iterate the vector
    for netstat_line in netstat {
        // Get the port of local_addr
        let local_port = get_port(&netstat_line.local_addr);
        // Get the port of foreign_addr
        let foreign_port = get_port(&netstat_line.foreign_addr);
        // If the port of local_addr or foreign_addr is equal to the port, push the processname into vector
        if is_local_addr {
            if local_port == port {
                processname_filtered.push(get_processname_by_pid(
                    netstat.clone(),
                    &netstat_line.pid,
                ));
            }
        } else {
            if foreign_port == port {
                processname_filtered.push(get_processname_by_pid(
                    netstat.clone(),
                    &netstat_line.pid,
                ));
            }
        }
    }
    // Return the vector
    processname_filtered
}