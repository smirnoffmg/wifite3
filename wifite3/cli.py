"""
Command-line interface for Wifite3
"""

import click
from rich.console import Console
from rich.panel import Panel
from rich.text import Text

console = Console()


@click.command()
@click.option("--interface", "-i", help="Network interface to use")
@click.option("--verbose", "-v", is_flag=True, help="Verbose output")
@click.option("--scan", is_flag=True, help="Scan for networks")
@click.option("--pmkid", is_flag=True, help="Capture PMKID from EAPOL frames")
@click.option(
    "--duration", "-d", default=30, help="Capture duration in seconds (default: 30)"
)
def main(interface, verbose, scan, pmkid, duration):
    """Wifite3 - Modern WiFi penetration testing tool"""

    # Display banner
    banner = Text("Wifite3", style="bold blue")
    banner.append("\nModern WiFi Penetration Testing Tool", style="dim")

    console.print(Panel(banner, title="Welcome", border_style="blue"))

    if verbose:
        console.print("[yellow]Verbose mode enabled[/yellow]")

    if scan:
        console.print("[green]Scanning for networks...[/green]")
        try:
            import _wifite3

            # Get available interfaces
            interfaces = _wifite3.NetworkScanner.get_interfaces()
            console.print(f"[blue]Available interfaces: {', '.join(interfaces)}[/blue]")

            # Use specified interface or first available
            target_interface = (
                interface if interface else (interfaces[0] if interfaces else "wlan0")
            )
            console.print(f"[blue]Using interface: {target_interface}[/blue]")

            # Perform scan
            scanner = _wifite3.NetworkScanner(target_interface)
            networks = scanner.scan()

            if networks:
                console.print(f"[green]Found {len(networks)} networks:[/green]")
                for network in networks:
                    console.print(
                        f"  • {network.ssid} ({network.bssid}) - {network.encryption}"
                    )
            else:
                console.print("[yellow]No networks found[/yellow]")

        except ImportError:
            console.print("[red]Rust module not available[/red]")
        except Exception as e:
            console.print(f"[red]Scanning failed: {e}[/red]")

    if pmkid:
        console.print(f"[green]Capturing PMKID for {duration} seconds...[/green]")
        try:
            import _wifite3

            # Get available interfaces
            interfaces = _wifite3.NetworkScanner.get_interfaces()
            console.print(f"[blue]Available interfaces: {', '.join(interfaces)}[/blue]")

            # Use specified interface or first available
            target_interface = (
                interface if interface else (interfaces[0] if interfaces else "wlan0")
            )
            console.print(f"[blue]Using interface: {target_interface}[/blue]")

            # Perform PMKID capture with SSID correlation
            scanner = _wifite3.NetworkScanner(target_interface)
            pmkid_captures = scanner.capture_pmkid_with_correlation(duration)

            if pmkid_captures:
                console.print(
                    f"[green]Captured {len(pmkid_captures)} PMKID(s):[/green]"
                )
                for capture in pmkid_captures:
                    console.print(f"  • {capture.get_summary()}")
                    console.print(f"    Hashcat format: {capture.get_hashcat_format()}")
            else:
                console.print("[yellow]No PMKID captured[/yellow]")

        except ImportError:
            console.print("[red]Rust module not available[/red]")
        except Exception as e:
            console.print(f"[red]PMKID capture failed: {e}[/red]")

    if not interface:
        console.print(
            "[yellow]No interface specified. Use -i to specify interface.[/yellow]"
        )


if __name__ == "__main__":
    main()
