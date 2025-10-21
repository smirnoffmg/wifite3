# PMKID Attack Documentation

## Overview

The PMKID (Pairwise Master Key Identifier) attack is a modern WiFi penetration testing technique that allows for efficient password cracking of WPA2 and WPA3 networks. Unlike traditional handshake capture methods, PMKID attacks can capture authentication data from a single EAPOL frame, making them more efficient and less detectable.

## What is PMKID?

PMKID is a 16-byte identifier used in WPA2/WPA3 authentication that is derived from the Pre-Shared Key (PSK) or passphrase. It's included in the RSN (Robust Security Network) Information Element of EAPOL frames during the authentication process.

### Key Characteristics

- **Size**: 16 bytes (128 bits)
- **Format**: Hexadecimal string representation
- **Location**: RSN Information Element in EAPOL frames
- **Purpose**: Authentication and key management
- **Derivation**: Based on PSK, SSID, and other network parameters

## Attack Process

### Prerequisites

- **Network Interface**: Wireless adapter capable of monitor mode
- **Target Network**: WPA2 or WPA3 secured network
- **Tools**: wifite3, hashcat, or similar cracking tools
- **Permissions**: Root/administrator access for network interface control

### Step-by-Step Process

1. **Interface Setup**
   ```bash
   # Enable monitor mode on wireless interface
   sudo airmon-ng start wlan0
   ```

2. **PMKID Capture**
   ```bash
   # Capture PMKID using wifite3
   wifite3 --pmkid --duration 60 -i wlan0mon
   ```

3. **Hashcat Format Generation**
   ```
   WPA*01*<PMKID>*<BSSID>*<CLIENT_MAC>*<SSID>
   ```

4. **Offline Cracking**
   ```bash
   # Crack PMKID using hashcat
   hashcat -m 22000 pmkid_hashes.txt wordlist.txt
   ```

## Technical Implementation

### EAPOL Frame Structure

```
┌─────────────────┬─────────────────┬─────────────────┐
│ Ethernet Header │ EAPOL Header    │ EAPOL Payload   │
│ (14 bytes)      │ (4 bytes)       │ (Variable)      │
└─────────────────┴─────────────────┴─────────────────┘
```

### RSN Information Element

```
┌─────────────┬─────────────┬─────────────┬─────────────┐
│ Element ID  │ Length      │ Version    │ Group Cipher│
│ (1 byte)    │ (1 byte)    │ (2 bytes)  │ (4 bytes)   │
└─────────────┴─────────────┴─────────────┴─────────────┘
┌─────────────┬─────────────┬─────────────┬─────────────┐
│ Pairwise    │ AKM Suite   │ RSN        │ PMKID Count │
│ Cipher Count│ Count       │ Capabilities│ (2 bytes)   │
│ (2 bytes)   │ (2 bytes)   │ (2 bytes)  │             │
└─────────────┴─────────────┴─────────────┴─────────────┘
┌─────────────┬─────────────┬─────────────┬─────────────┐
│ PMKID List  │ (16 bytes   │ per PMKID) │             │
└─────────────┴─────────────┴─────────────┴─────────────┘
```

### PMKID Extraction Process

1. **Frame Detection**: Identify EAPOL frames (EtherType 0x888E)
2. **EAPOL Validation**: Verify EAPOL-Key frame type (Type 3)
3. **RSN IE Parsing**: Locate RSN Information Element (ID 48)
4. **PMKID Extraction**: Extract PMKID from RSN IE payload
5. **Format Conversion**: Convert to hashcat-compatible format

## Effectiveness

### Success Factors

- **Network Activity**: Requires active client authentication
- **WPA2/WPA3 Support**: Only works with supported security protocols
- **PMKID Availability**: Depends on AP and client implementation
- **Password Strength**: Effectiveness inversely related to password complexity

### Performance Metrics

- **Capture Time**: Typically 1-5 minutes for active networks
- **Success Rate**: 80-95% for networks with active clients
- **Detection Risk**: Lower than traditional deauth attacks
- **Cracking Speed**: 500k+ passwords/second on modern hardware

## Countermeasures

### Network Defense

1. **Strong Passwords**
   - Use complex, random passphrases (20+ characters)
   - Avoid dictionary words and common patterns
   - Include numbers, symbols, and mixed case

2. **Network Configuration**
   - Enable WPA3 when available
   - Use enterprise authentication (802.1X)
   - Implement MAC address filtering
   - Disable WPS if not needed

3. **Monitoring**
   - Monitor for unusual authentication patterns
   - Use intrusion detection systems
   - Regular security audits
   - Network segmentation

### Detection Methods

- **Traffic Analysis**: Monitor for excessive EAPOL frames
- **Authentication Logs**: Review AP authentication logs
- **Network Monitoring**: Use tools like Wireshark for analysis
- **Behavioral Analysis**: Detect unusual network patterns

## Legal and Ethical Considerations

### ⚠️ **IMPORTANT LEGAL NOTICE**

**ONLY use PMKID attacks on networks you own or have explicit written permission to test. Unauthorized network testing is illegal in most jurisdictions and may result in criminal charges.**

### Ethical Guidelines

1. **Authorization**: Always obtain written permission before testing
2. **Scope**: Limit testing to authorized networks only
3. **Documentation**: Maintain records of authorized testing
4. **Responsible Disclosure**: Report vulnerabilities to network owners
5. **Legal Compliance**: Follow local laws and regulations

### Responsible Use

- **Penetration Testing**: Use only in authorized security assessments
- **Research**: Academic research with proper oversight
- **Education**: Training with controlled environments
- **Defense**: Understanding attack vectors for better protection

## Tools and Implementation

### wifite3 Implementation

```python
# PMKID capture using wifite3
from wifite3 import NetworkScanner

scanner = NetworkScanner("wlan0")
pmkid_captures = scanner.capture_pmkid(duration_seconds=60)

for capture in pmkid_captures:
    print(f"SSID: {capture.ssid}")
    print(f"BSSID: {capture.bssid}")
    print(f"PMKID: {capture.pmkid}")
    print(f"Hashcat Format: {capture.get_hashcat_format()}")
```

### Command Line Usage

```bash
# Basic PMKID capture
wifite3 --pmkid --duration 60

# With specific interface
wifite3 --pmkid -i wlan0 --duration 120

# Verbose output
wifite3 --pmkid --verbose --duration 30
```

### Hashcat Integration

```bash
# Prepare hashcat input file
echo "WPA*01*<PMKID>*<BSSID>*<CLIENT_MAC>*<SSID>" > pmkid_hashes.txt

# Run hashcat attack
hashcat -m 22000 -a 0 pmkid_hashes.txt wordlist.txt

# Use rules for better results
hashcat -m 22000 -a 0 pmkid_hashes.txt wordlist.txt -r rules/best64.rule
```

## Troubleshooting

### Common Issues

1. **No PMKID Captured**
   - Ensure network has active clients
   - Check interface is in monitor mode
   - Verify WPA2/WPA3 support
   - Increase capture duration

2. **Interface Not Found**
   - Check interface name (wlan0, wlan1, etc.)
   - Ensure interface supports monitor mode
   - Verify driver compatibility
   - Check permissions (root access required)

3. **Hashcat Cracking Fails**
   - Verify hashcat format is correct
   - Check wordlist quality
   - Ensure sufficient computational resources
   - Try different attack modes

### Debugging Tips

- **Verbose Output**: Use `--verbose` flag for detailed information
- **Packet Analysis**: Use Wireshark to analyze captured packets
- **Interface Testing**: Test with known networks first
- **Log Analysis**: Review system logs for errors

## Performance Optimization

### Capture Optimization

- **Duration**: Balance between capture time and success rate
- **Interface**: Use dedicated wireless adapters for better performance
- **Location**: Position closer to target network
- **Timing**: Capture during peak usage hours

### Cracking Optimization

- **Hardware**: Use GPU acceleration when available
- **Wordlists**: Use targeted wordlists for better success rates
- **Rules**: Apply transformation rules for common patterns
- **Distributed**: Use multiple systems for large wordlists

## Advanced Techniques

### PMKID Correlation

- **SSID Mapping**: Correlate PMKID with beacon frames for SSID identification
- **Client Tracking**: Monitor client authentication patterns
- **Network Profiling**: Build network behavior profiles

### Automation

- **Scripted Capture**: Automate PMKID capture processes
- **Batch Processing**: Handle multiple networks simultaneously
- **Integration**: Combine with other WiFi security tools
- **Reporting**: Generate automated security reports

## References

### Technical Standards

- **IEEE 802.11i**: Wireless security standard
- **RFC 3748**: EAP (Extensible Authentication Protocol)
- **RFC 4017**: EAP Method Requirements for Wireless LANs
- **WPA3 Specification**: Latest WiFi security standard

### Security Research

- **Hashcat Team**: PMKID attack development
- **WiFi Security Research**: Academic papers and studies
- **Penetration Testing**: Industry best practices
- **Vulnerability Databases**: CVE and security advisories

### Tools and Resources

- **wifite3**: Modern WiFi penetration testing tool
- **hashcat**: Password cracking tool
- **aircrack-ng**: WiFi security assessment suite
- **Wireshark**: Network protocol analyzer

## Conclusion

The PMKID attack represents a significant advancement in WiFi security testing, offering more efficient and less detectable password recovery compared to traditional methods. However, its power comes with responsibility - it must only be used in authorized security assessments and educational contexts.

Understanding PMKID attacks is crucial for both offensive security professionals and network defenders. By implementing proper countermeasures and monitoring, organizations can protect their networks from these sophisticated attacks.

Remember: **Always obtain proper authorization before conducting any security testing. Unauthorized network testing is illegal and unethical.**
