import React, { ReactNode } from "react";
import styled, { createGlobalStyle } from "styled-components";
import { Header } from "../molecules/header.component";
import SEO, { Props as SEOProps } from "../atoms/seo.component";
import { fontFamily } from "../atoms/typography.component";
import { Stylable } from "../../types/component.types";
import { Footer } from "../molecules/footer.component";
import { maxWidth, spacing } from "../../utils/spacing.utils";

type Props = SEOProps &
  Stylable & {
    children: ReactNode;
  };

const GlobalStyles = createGlobalStyle`
  *, *:before, *:after {
    box-sizing: border-box;

    &:focus:not(:focus-visible) { outline: none }
  }

  html,
  body {
    width: 100%;
    height: 100%;
    margin: 0;
    padding: 0;
  }

  body {
    position: relative;
    font-style: normal;
    font-weight: 400;
    font-family: ${fontFamily};
    font-size: 16px;
    line-height: 1.4;
  }

  @media(prefers-reduced-motion: reduce) {
    *,
    *:before,
    *:after {
      transition: none !important;
      animation: none !important;
    }
  }
`;

const RawLayout = ({ className, children, title }: Props) => {
  return (
    <div className={className}>
      <GlobalStyles />
      <Header>{title}</Header>
      <SEO title={title} />
      <main>{children}</main>
      <Footer />
    </div>
  );
};

export const Layout = styled(RawLayout)`
  main {
    width: 90vw;
    max-width: ${maxWidth}px;
    margin: ${spacing(2)}px auto;
  }
`;
