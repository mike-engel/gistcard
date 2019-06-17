import React from "react";
import Helmet from "react-helmet";

type MetaProps = JSX.IntrinsicElements["meta"];

export interface Props {
  description?: string;
  lang?: string;
  meta?: MetaProps[];
  title: string;
}

const SEO = ({ description, lang, meta, title }: Props) => {
  const metaDescription = description;

  return (
    <Helmet
      htmlAttributes={{
        lang
      }}
      title={title}
      meta={[
        {
          name: `description`,
          content: metaDescription
        },
        {
          property: `og:title`,
          content: title
        },
        {
          property: `og:description`,
          content: metaDescription
        },
        {
          property: `og:type`,
          content: `website`
        },
        {
          name: `twitter:card`,
          content: `summary`
        },
        {
          name: `twitter:creator`,
          content: `@beardfury`
        },
        {
          name: `twitter:title`,
          content: title
        },
        {
          name: `twitter:description`,
          content: metaDescription
        }
        // @ts-ignore
      ].concat(meta)}
    />
  );
};

SEO.defaultProps = {
  lang: `en`,
  meta: [],
  description: ``
};

export default SEO;
