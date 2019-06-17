import React, { memo } from "react";
import styled from "styled-components";
import { Stylable } from "../../types/component.types";
import { ExternalLink, Text } from "../atoms/typography.component";
import { maxWidth, spacing } from "../../utils/spacing.utils";

type Props = Stylable;

export const RawFooter = memo(({ className }: Props) => (
  <footer className={className}>
    <Text level={5}>
      ©{new Date().getFullYear()}, Built by
      {` `}
      <ExternalLink href="https://mike-engel.com">Mike Engel</ExternalLink>
      {" | "}
      <ExternalLink href="https://github.com/mike-engel/gistcard">View on GitHub</ExternalLink>
    </Text>
  </footer>
));

export const Footer = styled(RawFooter)`
  display: flex;
  align-items: center;
  justify-content: center;
  padding: ${spacing(2)}px 0;

  ${Text} {
    width: 90vw;
    max-width: ${maxWidth}px;
  }
`;
