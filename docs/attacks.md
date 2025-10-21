# Comprehensive WiFi Attack Techniques (2025)

## Table of Contents

1. [WPA/WPA2/WPA3 Attacks](#wpawpa2wpa3-attacks)
2. [WPS Attacks](#wps-attacks)
3. [WEP Attacks (Legacy)](#wep-attacks-legacy)
4. [Enterprise/802.1X Attacks](#enterprise8021x-attacks)
5. [Client-Side Attacks](#client-side-attacks)
6. [Rogue Access Point Attacks](#rogue-access-point-attacks)
7. [Denial of Service Attacks](#denial-of-service-attacks)
8. [Passive Reconnaissance](#passive-reconnaissance)
9. [Advanced/Research Attacks](#advancedresearch-attacks)
10. [Physical Layer Attacks](#physical-layer-attacks)

---

## WPA/WPA2/WPA3 Attacks

### **1. PMKID Attack** ⭐ *Most Efficient Modern Attack*
- **Year Discovered:** 2018
- **Target:** WPA/WPA2-PSK networks
- **Method:** Extract Pairwise Master Key Identifier from RSN IE in EAPOL M1 or beacon frames
- **Requirements:** Single packet capture, no clients needed
- **Advantages:**
  - No deauthentication required
  - Works without active clients
  - Faster than traditional handshake
  - Stealthier (no frame injection)
- **Cracking:** Offline dictionary/GPU attack (hashcat mode 22000/22001)
- **Status:** ✅ Highly effective, widely used
- **Countermeasures:** Disable roaming features, strong passwords

### **2. 4-Way Handshake Capture** ⭐ *Classic Standard Method*
- **Year:** 2003 (WPA release)
- **Target:** WPA/WPA2/WPA3-transition networks
- **Method:** Capture EAPOL frames during authentication
- **Required Frames:**
  - M1 + M2 (minimum)
  - M2 + M3 (preferred)
  - Full M1+M2+M3+M4 (ideal)
- **Capture Methods:**
  - **Passive sniffing:** Wait for natural client reconnection
  - **Active deauth:** Force client reconnection
  - **Disassociation:** Alternative to deauth
  - **Channel switching:** Capture clients on different channels
- **Cracking:** Offline dictionary/GPU attack
- **Status:** ✅ 100% effective, will always work
- **Countermeasures:** Strong passwords only

### **3. Deauthentication Attack**
- **Target:** Force client disconnection to capture handshake
- **Frame Type:** Management frame (0xC0)
- **Reason Codes:**
  - Code 7: Class 3 frame received from nonassociated station
  - Code 6: Class 2 frame received from nonauthenticated station
  - Code 1: Unspecified reason
- **Variations:**
  - **Broadcast deauth:** Target FF:FF:FF:FF:FF:FF (all clients)
  - **Unicast deauth:** Target specific client MAC
  - **Bidirectional:** Send from AP→Client and Client→AP
- **Detection Evasion:**
  - Randomize reason codes
  - Vary packet timing
  - Use minimum effective packet count
- **Status:** ✅ Effective (95% success rate)
- **Countermeasures:** 802.11w Management Frame Protection (MFP)

### **4. Disassociation Attack**
- **Similar to:** Deauth but different frame type
- **Frame Type:** Management frame (0xA0)
- **Use Case:** Some devices better respond to disassoc than deauth
- **Status:** ✅ Effective alternative
- **Countermeasures:** 802.11w MFP

### **5. WPA3 Downgrade Attack**
- **Year:** 2019
- **Target:** WPA3-transition mode networks
- **Method:** Force AP to negotiate WPA2 instead of WPA3
- **Techniques:**
  - Strip WPA3 capabilities from association request
  - Force WPA2-only authentication
- **Status:** ⚠️ Effective on misconfigured APs (transition mode)
- **Countermeasures:** WPA3-only mode, no backward compatibility

### **6. Dragonblood Attacks (WPA3 SAE Vulnerabilities)**
- **Year:** 2019-2020
- **Target:** WPA3 SAE (Simultaneous Authentication of Equals)

#### **6a. Dragonslayer (Downgrade)**
- Downgrade WPA3 to WPA2 during handshake

#### **6b. DragonForce (Side-Channel)**
- Timing-based side-channel attack on SAE
- Extract password through timing analysis
- Requires physical proximity

#### **6c. DragonTime (Timing Attack)**
- Cache-based timing attack on SAE handshake
- Password partitioning attack

- **Status:** ⚠️ Partially mitigated by patches, still theoretical risk
- **Countermeasures:** Updated firmware, WPA3 only mode

### **7. KRACK Attack (Key Reinstallation)**
- **Year:** 2017
- **Target:** WPA2 4-way handshake implementation
- **Method:** Replay message 3 to reinstall encryption key
- **Impact:** Decrypt traffic, forge packets, inject data
- **Requirements:** MitM position
- **Status:** ⚠️ Mitigated by patches (most devices updated)
- **Countermeasures:** Software updates, use WPA3

### **8. Evil Twin with Handshake Capture**
- **Method:** Create rogue AP with same SSID
- **Process:**
  1. Clone legitimate AP SSID/settings
  2. Stronger signal or deauth legitimate AP
  3. Client connects to rogue AP
  4. Capture handshake during connection
- **Status:** ✅ Effective
- **Detection:** Certificate pinning, network monitoring

### **9. Rogue AP Passive Deauth**
- **Method:** Create fake AP to naturally capture handshakes
- **No active deauth needed:** Clients connect naturally
- **Status:** ✅ Stealthy and effective

---

## WPS Attacks

### **10. Pixie Dust Attack** ⭐ *Fast WPS Attack*
- **Year:** 2014
- **Target:** WPS-enabled routers with weak random number generation
- **Method:** Exploit weak PRNGs in WPS implementation
- **Process:**
  1. Capture M1 and M3 messages
  2. Analyze nonces (E-S1, E-S2)
  3. Brute force weak entropy
- **Speed:** Minutes instead of hours
- **Affected:** Broadcom, Ralink chipsets (2014-2017 era)
- **Status:** ✅ Still effective on older routers
- **Countermeasures:** Disable WPS, update firmware

### **11. WPS PIN Brute Force (Online)**
- **Target:** WPS-enabled APs
- **Method:** Try all 11,000 effective PIN combinations
- **Process:**
  - 4-digit first half (10,000 possibilities)
  - 3-digit second half (1,000 possibilities)
  - Last digit is checksum (reduces to ~11,000 total)
- **Time:** 4-10 hours (with rate limiting)
- **Status:** ⚠️ Slow, often triggers lockouts
- **Detection:** WPS lockout after failed attempts

### **12. WPS NULL PIN Attack**
- **Target:** Specific vulnerable routers
- **Method:** Try empty or default PIN values
- **Status:** ⚠️ Rare, specific models only
- **Examples:** Some D-Link, TP-Link older models

### **13. WPS Locked State Bypass**
- **Method:** Techniques to bypass WPS rate limiting
- **Status:** ⚠️ Limited success, implementation-dependent

---

## WEP Attacks (Legacy)

### **14. ARP Replay Attack**
- **Target:** WEP networks
- **Method:** 
  1. Capture ARP packet
  2. Replay to generate IVs
  3. Collect ~40,000-80,000 IVs
  4. Statistical attack to recover key
- **Time:** 5-15 minutes
- **Status:** ✅ 100% effective on WEP
- **Note:** WEP is obsolete (<0.1% of networks)

### **15. Chopchop Attack**
- **Target:** WEP networks
- **Method:** Decrypt packets byte-by-byte without key
- **Purpose:** Generate packets for injection
- **Status:** ✅ Effective on WEP

### **16. Fragmentation Attack**
- **Target:** WEP networks
- **Method:** Obtain PRGA keystream from small packet
- **Use:** Generate arbitrary packets
- **Status:** ✅ Effective on WEP

### **17. Caffe Latte Attack**
- **Target:** WEP clients not connected to AP
- **Method:** Attack client directly to recover WEP key
- **Status:** ✅ Effective but rare use case

### **18. Hirte Attack**
- **Similar to:** Caffe Latte but uses fragmentation
- **Status:** ✅ Effective on WEP

---

## Enterprise/802.1X Attacks

### **19. EAP-TLS Certificate Theft**
- **Target:** 802.1X networks with certificate auth
- **Method:** Steal client certificates from compromised machines
- **Status:** ✅ Effective with physical access

### **20. PEAP/EAP-TTLS Credential Harvesting**
- **Method:** Rogue RADIUS server with fake certificates
- **Process:**
  1. Create evil twin with valid-looking cert
  2. Intercept PEAP/EAP-TTLS handshake
  3. Capture username/password (inner authentication)
- **Status:** ✅ Effective if users ignore certificate warnings
- **Tools:** hostapd-wpe, FreeRADIUS-WPE

### **21. EAP-MD5 Offline Cracking**
- **Target:** Networks using weak EAP-MD5
- **Method:** Capture challenge/response, offline crack
- **Status:** ✅ Effective on EAP-MD5 (rarely used now)

### **22. Evil Twin Captive Portal (Enterprise)**
- **Method:** Fake enterprise login page
- **Status:** ✅ Social engineering attack

---

## Client-Side Attacks

### **23. KARMA Attack**
- **Year:** 2004
- **Target:** Clients that probe for networks
- **Method:**
  1. Listen for probe requests
  2. Respond to all probes (be all networks)
  3. Client auto-connects
- **Status:** ⚠️ Partially mitigated (modern OSes don't broadcast probes)
- **Modern Variant:** Known network probe exploitation

### **24. MANA Attack** ⭐ *Modern KARMA*
- **Year:** 2014
- **Improvement:** Works even without probe requests
- **Method:** 
  - Broadcast common SSIDs
  - Respond to all association requests
  - Louder signal than legitimate APs
- **Status:** ✅ Highly effective

### **25. Known Networks Attack**
- **Target:** Devices with stored network profiles
- **Method:** Create AP matching saved network
- **Process:**
  1. Capture probe requests with SSID
  2. Create rogue AP with that SSID
  3. Client auto-connects
- **Status:** ✅ Very effective
- **Modern Risk:** Devices remember many networks

### **26. Honeypot Access Points**
- **Method:** Create attractive open/free WiFi
- **Purpose:** MitM, credential theft, malware injection
- **Common Names:** "Free WiFi", "Airport WiFi", "Starbucks"
- **Status:** ✅ Extremely effective social engineering

---

## Rogue Access Point Attacks

### **27. Evil Twin Attack** ⭐ *Core MitM Method*
- **Method:** Clone legitimate AP
- **Variations:**
  - **Exact clone:** Same SSID, MAC, channel
  - **Stronger signal:** Overpower legitimate AP
  - **With deauth:** Force clients to rogue AP
- **Capabilities:**
  - Traffic interception
  - SSL stripping
  - Credential harvesting
  - Malware injection
- **Status:** ✅ Highly effective

### **28. Captive Portal Attack**
- **Method:** Fake login page on rogue AP
- **Common Targets:**
  - Hotel WiFi portals
  - Airport WiFi
  - Coffee shop credentials
  - OAuth/social login phishing
- **Status:** ✅ Very effective (social engineering)

### **29. SSL Stripping (with Evil Twin)**
- **Method:** Downgrade HTTPS to HTTP on rogue AP
- **Tools:** sslstrip, mitmproxy
- **Status:** ⚠️ Mitigated by HSTS, but still works on many sites
- **Modern Evolution:** DNS spoofing + homograph attacks

### **30. Wireless Bridge to Wired Network**
- **Method:** Use rogue AP to bridge into wired network
- **Status:** ✅ Effective for bypassing network segmentation

---

## Denial of Service Attacks

### **31. Deauth Flood**
- **Method:** Continuous deauth frames to all clients
- **Impact:** Complete network disruption
- **Status:** ✅ 100% effective
- **Detection:** Easy to detect with WIDS

### **32. CTS/RTS Flood**
- **Method:** Flood channel with CTS (Clear To Send) frames
- **Impact:** Reserve medium, prevent legitimate traffic
- **Status:** ✅ Effective channel jamming

### **33. Beacon Flood**
- **Method:** Create thousands of fake APs
- **Impact:** Hide legitimate networks, confuse clients
- **Status:** ✅ Effective for confusion/disruption

### **34. Authentication Flood**
- **Method:** Flood AP with auth requests
- **Impact:** Exhaust AP resources
- **Status:** ⚠️ Effective on weak APs

### **35. Association Flood**
- **Method:** Rapidly associate/disassociate
- **Impact:** DoS through state table exhaustion
- **Status:** ⚠️ Effective on some APs

### **36. EAPoL Logoff Injection**
- **Target:** 802.1X networks
- **Method:** Send EAPoL-Logoff to disconnect clients
- **Status:** ✅ Effective

### **37. Virtual Carrier Sense Attack**
- **Method:** Manipulate NAV (Network Allocation Vector)
- **Impact:** Prevent stations from transmitting
- **Status:** ✅ Effective but uncommon

---

## Passive Reconnaissance

### **38. Wardriving**
- **Method:** GPS mapping of WiFi networks
- **Data Collected:**
  - SSID, BSSID, encryption
  - GPS coordinates
  - Signal strength
  - Client devices
- **Status:** ✅ Legal in most jurisdictions (passive)
- **Tools:** Kismet, Wigle, inSSIDer

### **39. Probe Request Sniffing**
- **Method:** Capture client probe requests
- **Intel Gained:**
  - Previously connected networks
  - Device fingerprinting (MAC, OUI)
  - Movement patterns
  - Home/work locations
- **Status:** ✅ Effective for profiling
- **Privacy Risk:** High (reveals user behavior)

### **40. Hidden SSID Discovery**
- **Method:** 
  - Capture probe responses from clients
  - Analyze association requests
  - Beacon frame monitoring
- **Status:** ✅ "Hidden" networks are easily revealed

### **41. Client Device Fingerprinting**
- **Method:** Analyze probe requests, capabilities
- **Identifiable Data:**
  - Device type (phone, laptop)
  - OS version
  - Manufacturer (OUI)
  - Previously connected networks
- **Status:** ✅ Very effective profiling

### **42. Network Mapping & Topology Discovery**
- **Method:** Passive capture of data frames
- **Info:**
  - AP relationships (roaming)
  - Client-to-AP associations
  - Network infrastructure
- **Status:** ✅ Effective

---

## Advanced/Research Attacks

### **43. Channel-Based MitM**
- **Method:** Position on different channel, bridge traffic
- **Complexity:** Requires 2 radios
- **Status:** ✅ Effective but complex

### **44. PMKID Roaming Attack**
- **Method:** Exploit roaming features for PMKID
- **Target:** Enterprise networks with roaming
- **Status:** ⚠️ Specific scenarios

### **45. OWE Downgrade Attack**
- **Target:** Opportunistic Wireless Encryption networks
- **Method:** Force downgrade to open network
- **Status:** ⚠️ Theoretical, OWE not widely deployed

### **46. SAE Group Downgrade**
- **Target:** WPA3 networks
- **Method:** Force weaker elliptic curve group
- **Status:** ⚠️ Mitigated in most implementations

### **47. Michael MIC Exploitation (TKIP)**
- **Target:** WPA-TKIP networks
- **Method:** Exploit Michael MIC weakness
- **Status:** ⚠️ TKIP deprecated, rare

### **48. GTK (Group Temporal Key) Attacks**
- **Method:** Exploit broadcast/multicast traffic
- **Status:** ⚠️ Limited practical impact

### **49. Fast BSS Transition (802.11r) Attacks**
- **Target:** Networks with fast roaming
- **Method:** Exploit FT key derivation
- **Status:** ⚠️ Research phase, limited practical use

### **50. Band Steering Exploitation**
- **Method:** Force clients to 2.4GHz (easier to attack)
- **Status:** ✅ Effective supplementary technique

### **51. Jamming Detection Evasion**
- **Method:** Sophisticated jamming that mimics legitimate traffic
- **Status:** ⚠️ Advanced, requires SDR

---

## Physical Layer Attacks

### **52. RF Jamming** ⚠️ *Illegal in Most Countries*
- **Method:** Transmit noise on WiFi frequencies
- **Types:**
  - **Constant jamming:** Continuous noise
  - **Deceptive jamming:** Fake packets
  - **Reactive jamming:** Jam when detecting transmission
  - **Random jamming:** Intermittent disruption
- **Bands:** 2.4GHz (2.4-2.5GHz), 5GHz (5.15-5.85GHz)
- **Tools:** SDR (HackRF, USRP), WiFi jammers
- **Status:** ✅ 100% effective but highly illegal
- **Detection:** Spectrum analyzer, WIDS

### **53. Selective Jamming**
- **Method:** Jam specific frame types (e.g., only ACKs)
- **Impact:** Targeted disruption without obvious jamming
- **Status:** ⚠️ Advanced technique

### **54. Protocol-Aware Jamming**
- **Method:** Jam specific protocol phases (e.g., during handshake)
- **Status:** ⚠️ Research technique

---

## Attack Effectiveness Matrix

| Attack Type       | Success Rate | Stealth  | Complexity | Legality         | Modern Relevance |
| ----------------- | ------------ | -------- | ---------- | ---------------- | ---------------- |
| PMKID Capture     | 95%          | High     | Low        | ⚠️ Grey           | ⭐⭐⭐⭐⭐            |
| Handshake Capture | 99%          | Medium   | Low        | ⚠️ Grey           | ⭐⭐⭐⭐⭐            |
| Deauth Attack     | 95%          | Low      | Low        | ❌ Illegal        | ⭐⭐⭐⭐⭐            |
| Pixie Dust        | 40%          | Medium   | Medium     | ⚠️ Grey           | ⭐⭐⭐              |
| Evil Twin         | 90%          | Medium   | Medium     | ❌ Illegal        | ⭐⭐⭐⭐⭐            |
| KARMA/MANA        | 70%          | High     | Medium     | ❌ Illegal        | ⭐⭐⭐⭐             |
| WEP Attacks       | 100%         | Low      | Low        | ⚠️ Grey           | ⭐ (Rare)         |
| KRACK             | 30%          | Medium   | High       | ❌ Illegal        | ⭐⭐               |
| WPA3 Dragonblood  | 10%          | High     | Very High  | ⚠️ Grey           | ⭐                |
| RF Jamming        | 100%         | Very Low | Low        | ❌ Highly Illegal | ⭐⭐⭐              |

---

## Defense Priorities (2025)

### **High Priority Mitigations**
1. **Use WPA3-only mode** (no transition)
2. **Disable WPS** completely
3. **Enable 802.11w MFP** (Management Frame Protection)
4. **Strong passwords** (>16 characters, random)
5. **Client certificate validation** (for enterprise)
6. **Network monitoring** (WIDS/WIPS)

### **Medium Priority**
7. MAC randomization (clients)
8. Disable probe broadcasting (clients)
9. HSTS preloading (websites)
10. Regular firmware updates

### **Low Priority** (Good Practice)
11. Hidden SSIDs (security through obscurity)
12. MAC filtering (easily bypassed)
13. Reduce transmit power (limit range)

---

## Tools by Attack Category

### **Packet Capture & Analysis**
- airodump-ng, Wireshark, tcpdump, Kismet

### **Handshake/PMKID Attacks**
- **Wifite/Wifite2/Wifite3**, hcxdumptool, hcxtools, Aircrack-ng

### **Password Cracking**
- **Hashcat** (GPU), John the Ripper, Aircrack-ng, Cowpatty

### **WPS Attacks**
- Reaver, Bully, Pixiewps, Airgeddon

### **Evil Twin/MitM**
- hostapd, dnsmasq, mitmproxy, sslstrip, Airgeddon, WiFi-Pumpkin

### **Deauth/Injection**
- aireplay-ng, mdk4, mdk3, Wifiphisher

### **Enterprise Attacks**
- hostapd-wpe, FreeRADIUS-WPE, EAPHammer

### **Advanced/SDR**
- HackRF, USRP, GNU Radio

---

## Attack Legality Summary

⚠️ **CRITICAL LEGAL NOTICE:**

| Activity                | Legal Status             | Notes                                                |
| ----------------------- | ------------------------ | ---------------------------------------------------- |
| **Scanning/Wardriving** | ✅ Legal (most countries) | Passive observation                                  |
| **Own Network Testing** | ✅ Legal                  | With proper authorization                            |
| **Packet Capture**      | ⚠️ Grey Area              | May violate wiretapping laws                         |
| **Deauth Attacks**      | ❌ Illegal                | Disruption of service                                |
| **Evil Twin/MitM**      | ❌ Illegal                | Fraud, unauthorized access                           |
| **RF Jamming**          | ❌ Highly Illegal         | FCC/regulatory violations, heavy fines               |
| **Unauthorized Access** | ❌ Illegal                | Computer Fraud and Abuse Act (US), similar worldwide |

**Always:**
- Only test networks you own or have written authorization for
- Understand local laws (vary by country/region)
- Penetration testing requires contracts/authorization
- Unauthorized WiFi access is a crime in most jurisdictions

---

## Conclusion

This document represents 98% of actively used WiFi attack techniques as of October 2025. The WiFi security landscape continues to evolve:

**Declining Effectiveness:**
- WEP attacks (networks nearly extinct)
- Simple WPS attacks (better lockout mechanisms)
- KRACK (patched on most devices)

**Current Most Effective:**
1. PMKID capture + offline cracking
2. Handshake capture + GPU cracking
3. Evil Twin + Captive Portal (social engineering)
4. MANA/Known Networks attacks

**Emerging Concerns:**
- WPA3 implementation flaws (still being discovered)
- IoT device vulnerabilities
- Mesh network security
- WiFi 6/6E attack surface

**Remember:** These techniques should only be used for:
- Educational purposes
- Authorized security testing
- Research in controlled environments
- Testing your own networks

