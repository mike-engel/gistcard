import React from "react";
import { Layout } from "../components/organisms/layout.component";
import {
  Text,
  ExternalLink,
  Span,
  FontWeight,
  Heading
} from "../components/atoms/typography.component";
import styled from "styled-components";
import { Stylable } from "../types/component.types";
import { Preview } from "../components/molecules/preview.component";
import { spacing } from "../utils/spacing.utils";

type Props = Stylable;

const RawIndexPage = ({ className }: Props) => (
  <Layout title="GistCard" className={className}>
    <Heading level={2}>What</Heading>
    <Text>
      <Span fontWeight={FontWeight.Bold}>GistCard</Span> is a better way to share gists (until
      github does this themselves 😉). It shows a snippet of the gist as a preview using{" "}
      <ExternalLink href="https://carbon.now.sh">carbon</ExternalLink> and includes a link to the
      gist.
    </Text>
    <Text>
      The original inspiration for this was from a{" "}
      <ExternalLink href="https://twitter.com/noopkat/status/1138552168372289537">
        tweet
      </ExternalLink>{" "}
      from <ExternalLink href="https://twitter.com/noopkat">Suz Hinton</ExternalLink>. It's a
      wonderful idea, and I fully expect this tool to only last for a little while until GitHub does
      it officially.
    </Text>
    <Preview />
    <Heading level={2}>Preview</Heading>
    <img src="/images/card_preview.png" alt="Preview of the twitter card" />
  </Layout>
);

export default styled(RawIndexPage)`
  ${Span} + ${ExternalLink} {
    margin-top: 0 !important;
  }

  ${Text} + ${Heading},
  ${Preview} + ${Heading} {
    margin-top: ${spacing(5)}px !important;
  }

  img {
    max-width: 504px;
    width: auto;
    height: auto;
  }
`;
