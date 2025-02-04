// docusaurus.config.js
module.exports = {
  title: "Secure OS Layer SDK",
  tagline: "A JavaScript SDK for interacting with the Secure OS Layer API",
  url: "https://github.com/yashasrnair/secure_os_layer", // Replace with your site URL
  baseUrl: "/secure-os-layer-docs/", // Adjust if needed
  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",
  favicon: "img/favicon.ico",
  organizationName: "Cybernix", // GitHub org/user name.
  projectName: "secure-os-layer-docs", // Repo name.
  themeConfig: {
    navbar: {
      title: "Secure OS Layer SDK",
      logo: {
        alt: "SDK Logo",
        src: "img/logo.svg",
      },
      items: [
        { to: "docs/getting-started", label: "Docs", position: "left" },
        { to: "docs/api", label: "API Reference", position: "left" },
        {
          href: "https://github.com/yashasrnair/secure_os_layer",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Docs",
          items: [
            { label: "Getting Started", to: "docs/getting-started" },
            { label: "API Reference", to: "docs/api" },
          ],
        },
        {
          title: "Community",
          items: [
            {
              label: "GitHub Issues",
              href: "https://github.com/yashasrnair/secure_os_layer/issues",
            },
          ],
        },
        {
          title: "More",
          items: [{ label: "Blog", to: "blog" }],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Your Name.`,
    },
  },
  presets: [
    [
      "classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          // You can add edit URL here if you plan to host your docs on GitHub.
        },
        blog: {
          showReadingTime: true,
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      }),
    ],
  ],
};
