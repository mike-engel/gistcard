import React, { useState, useEffect, useRef, SetStateAction, Dispatch } from "react";
import styled from "styled-components";
import {
  Text,
  Heading,
  Span,
  FontWeight,
  ExternalLink,
  fontFamily,
  fontSize
} from "../atoms/typography.component";
import { Stylable } from "../../types/component.types";
import { spacing } from "../../utils/spacing.utils";
import { darkGrey, lightGrey, black, hexToRgba, white, pSBC } from "../atoms/color.component";

type Props = Stylable;

export const copyText = (
  input: HTMLInputElement | null,
  setState: Dispatch<SetStateAction<boolean>>
) => {
  if (!input) return;

  const tempTextarea = document.createElement("textarea");

  tempTextarea.value = `https://gistcard.now.sh/${input.value}`;
  tempTextarea.style.position = "absolute";
  tempTextarea.style.width = "0px";
  tempTextarea.style.height = "0px";
  tempTextarea.style.top = "0px";
  tempTextarea.style.bottom = "0px";

  document.body.appendChild(tempTextarea);

  tempTextarea.select();

  setState(true);

  document.execCommand("copy");

  document.body.removeChild(tempTextarea);

  input.focus();
};

export const RawPreview = ({ className }: Props) => {
  const [id, setId] = useState("");
  const [copied, setCopied] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);
  const url = `https://gistcard.now.sh/${id}`;

  useEffect(() => {
    if (!inputRef.current) return;

    inputRef.current.focus();
  }, []);

  useEffect(() => {
    if (!copied) return;

    const id = setTimeout(() => {
      setCopied(false);
    }, 5000);

    return () => {
      clearTimeout(id);
    };
  }, [copied]);

  return (
    <div className={className}>
      <Heading level={2}>Usage</Heading>
      <Text>
        <Span fontWeight={FontWeight.Bold}>GistCard</Span> is used by adding the gist ID or gist url
        after <code>https://gistcard.now.sh</code>. Clicking on that link will rediredct any user to
        the actual gist page.
      </Text>
      <fieldset>
        <Heading level={3}>Create a GistCard</Heading>
        <label htmlFor="id">
          <Span color={darkGrey} level={5}>
            Gist ID or URL
          </Span>
          <input
            ref={inputRef}
            type="text"
            name="id"
            id="id"
            placeholder="2eb2a78488e7470bdb0377f4387f8240"
            value={id}
            onChange={evt => setId(evt.currentTarget.value)}
          />
        </label>
        <pre aria-live="polite">
          <ExternalLink href={url}>{url}</ExternalLink>
          <button onClick={() => copyText(inputRef.current, setCopied)}>
            {copied ? "Copied!" : "Copy link"}
          </button>
        </pre>
      </fieldset>
    </div>
  );
};

export const Preview = styled(RawPreview)`
  margin-top: ${spacing(5)}px !important;
  padding-top: 1px;

  fieldset {
    border: 1px solid ${pSBC(0.75, lightGrey)};
    background: ${white};
    box-shadow: 0 2px 4px ${hexToRgba(black, 0.15)};
    border-radius: ${spacing(0.5)}px;
    padding: ${spacing(2)}px;
  }

  label {
    display: block;
    margin: 0 0 ${spacing(2)}px 0;

    span {
      display: block;
    }
  }

  label + pre,
  label input {
    width: 100%;
    margin-top: ${spacing(1)}px !important;
    padding: ${spacing(1.5)}px ${spacing(2)}px;
    border-radius: ${spacing(0.5)}px;
    border: 1px solid ${pSBC(0.6, lightGrey)};
    font-family: ${fontFamily};
    font-size: ${fontSize.level4};
  }

  label + pre {
    background: #eee;
  }

  pre {
    display: flex;
    align-items: top;
    justify-content: space-between;
    white-space: pre-wrap;
    margin: 0;

    ${ExternalLink} {
      flex-shrink: 1;
    }
  }

  button {
    appearance: none;
    font-family: ${fontFamily};
    font-size: ${fontSize.level4};
    color: ${black};
    border: none;
    font-weight: ${FontWeight.Bold};
    white-space: nowrap;

    @media (hover) {
      &:hover {
        cursor: pointer;
      }
    }
  }
`;
