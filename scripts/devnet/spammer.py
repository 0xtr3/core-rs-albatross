from typing import Optional
from jinja2 import Environment

from node import Node, NodeType
from topology_settings import TopologySettings


class Spammer(Node):
    """
    Devnet spammer node

    :param name: Name of the validator node.
    :type name: str
    :param listen_port: Port this node will be listening to connections.
    :type listen_port: int
    :param topology_settings: General topology settings
    :type topology_settings: TopologySettings
    :param tpb: Transactions per block to send when running the binary
    :type tpb: int
    :param sync_mode: The node sync mode (history, full or light)
    :type sync_mode: str
    :param metrics: Optional metrics settings
    :type metrics: Optional[dict]
    """

    def __init__(self, name: str, listen_port: int,
                 topology_settings: TopologySettings, tpb: int,
                 sync_mode: str = 'history', metrics: Optional[dict] = None):
        self.address = "NQ40 GCAA U3UX 8BKD GUN0 PG3T 17HA 4X5H TXVE"
        super(Spammer, self).__init__(NodeType.SPAMMER,
                                      name, "nimiq-spammer", listen_port,
                                      topology_settings, sync_mode, metrics,
                                      nimiq_exec_extra_args=['-t', str(tpb)])

    def get_address(self):
        """
        Gets the spammer address

        :return: The validator address keypair.
        :rtype: str
        """
        return self.address

    def get_tpb(self):
        """
        Gets the number of transactions per block

        :return: The number of transactions per block.
        :rtype: int
        """
        return self.tpb

    def generate_config_files(self, jinja_env: Environment,
                              seed_addresses: list):
        """
        Generates configuration file

        :param jinja_env: Jinja2 environment for template rendering
        :type jinja_env: Environment
        :param seed_addresses: List of seed addresses in multiaddress format
            for the configuration file
        :type seed_addresses: List of strings
        """
        # Read and render the TOML template
        template = jinja_env.get_template("node_conf.toml.j2")
        metrics = self.get_metrics()
        loki_settings = self.topology_settings.get_loki_settings()
        if loki_settings is not None:
            loki_settings = loki_settings.format_for_config_file()
            loki_settings['extra_fields']['nimiq_node'] = self.name
        if metrics is not None:
            content = template.render(
                min_peers=3, port=self.get_listen_port(),
                state_path=self.get_state_dir(),
                sync_mode=self.get_sync_mode(), seed_addresses=seed_addresses,
                metrics=metrics, spammer=True, loki=loki_settings)
        else:
            content = template.render(
                min_peers=3, port=self.get_listen_port(),
                state_path=self.get_state_dir(),
                sync_mode=self.get_sync_mode(), seed_addresses=seed_addresses,
                spammer=True, loki=loki_settings)
        filename = self.get_conf_toml()
        with open(filename, mode="w", encoding="utf-8") as message:
            message.write(content)
