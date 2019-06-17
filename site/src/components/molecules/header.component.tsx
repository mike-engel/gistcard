import React, { ReactNode, memo } from "react";
import styled from "styled-components";
import { Heading, FontWeight } from "../atoms/typography.component";
import { Stylable } from "../../types/component.types";
import { spacing, maxWidth } from "../../utils/spacing.utils";
import { darkGrey, black } from "../atoms/color.component";

type Props = Stylable & {
  children?: ReactNode;
};

export const RawHeader = memo(({ children = "GistCard", className }: Props) => (
  <header className={className}>
    <Heading fontWeight={FontWeight.Heavy}>{children}</Heading>
    <Heading level={2} displayLevel={3} color={darkGrey} fontWeight={FontWeight.Regular}>
      Show better gist previews in twitter with a code snippet and a link to the gist
    </Heading>
  </header>
));

export const Header = styled(RawHeader)`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: ${spacing(3)}px 0;

  ${Heading} {
    width: 90vw;
    max-width: ${maxWidth}px;
  }

  ${Heading} + ${Heading} {
    margin-top: 0 !important;
  }

  ${Heading}:first-child {
    border-top: ${spacing(0.5)}px solid ${black};
    padding-top: ${spacing(0.5)}px;
  }
`;
